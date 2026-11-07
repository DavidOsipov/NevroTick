"""Bounded lexical Slint text-binding trace, independent of catalog definitions.

This follows named component/property bindings, inheritance, root/self/element
references and a fixed public native text-property allowlist. It is source trace
only: not a Slint compiler, general value/branch analysis, UIA or Rust call resolution.
"""
from bisect import bisect_right
from collections import defaultdict,deque
import re

MAX_NODES=8192
MAX_BINDINGS=16384
MAX_EDGES=65536
MAX_BRACE_DEPTH=128
COMPONENT=re.compile(r'\b(?:export\s+)?component\s+([A-Z][A-Za-z0-9_]*)\s+inherits\s+([A-Z][A-Za-z0-9_]*)\s*\{')
PROPERTY=re.compile(r'(?<![.\w-])([a-z][a-z0-9-]*)\s*(?::|<=>)')
COPY=re.compile(r'\b((?:[A-Z][A-Za-z0-9_]*)?Copy)((?:\.[a-z][a-z0-9-]*)+)')
LOCAL=re.compile(r'\b(root|self|[a-z][a-z0-9-]*)\.([a-z][a-z0-9-]*)\b')
COPY_SOURCE_PATHS=frozenset('crates/nevrotick-ui/ui/'+name+'-copy.slint' for name in ['fixture','preferences','source','recovery','shell','policy','inspection','diagnostic','raw','localization'])
# Fixed first-party product entry, independent of catalog metadata. A new entry
# requires explicit source-owner review together with its build/host receivers.
PRODUCT_ROOTS=frozenset({('crates/nevrotick-ui/ui/app.slint','AppWindow')})
NATIVE_TEXT={
 'Text':{'text'},'Button':{'text'},'CheckBox':{'text'},
 'ComboBox':{'model','current-value'},'LineEdit':{'text','placeholder-text'},
 'TextEdit':{'text'},'Window':{'title'},'MenuItem':{'title'},'Menu':{'title'},
}


def outside_parameters(clean,matches):
    """Ignore property-like colons inside function/callback parameter lists."""
    at=0;parentheses=0
    for found in matches:
        for token in clean[at:found.start()]:
            if token=='(':parentheses+=1
            elif token==')':parentheses-=1
            if parentheses<0:raise ValueError('Slint parameter nesting')
        at=found.start()
        if not parentheses:yield found


def _expression_end(clean,start):
    at=start;parentheses=0;brackets=0;braces=0
    while at<len(clean):
        token=clean[at]
        if token in ';}' and not parentheses and not brackets and not braces:return at
        if token=='(':parentheses+=1
        elif token==')':parentheses-=1
        elif token=='[':brackets+=1
        elif token==']':brackets-=1
        elif token=='{':braces+=1
        elif token=='}':braces-=1
        if min(parentheses,brackets,braces)<0:raise ValueError('Slint binding nesting')
        at+=1
    return at


def _space_before(clean,at):
    while at and clean[at-1].isspace():at-=1
    return at


def _name_before(clean,at):
    """Read one complete identifier token, never a suffix of a longer name."""
    end=at
    while at and(clean[at-1].isalnum()or clean[at-1]in '_-'):at-=1
    return at,clean[at:end]


def _instance_header(clean,opening):
    """Read the type, optional identifier and preceding position of an instance.

    Each opening brace consumes only its immediately preceding whitespace/name
    tokens. Comments/strings are already masked; no fixed lookback or repeated
    whole-prefix regex can hide a long but source-budgeted header.
    """
    at,typ=_name_before(clean,_space_before(clean,opening))
    if re.fullmatch(r'[A-Z][A-Za-z0-9_]*',typ)is None:return None,None,None
    at=_space_before(clean,at);identifier=None
    if at and clean[at-1]=='=':
        if at<2 or clean[at-2:at]!=':=':raise ValueError('unsupported Slint instance header')
        at,identifier=_name_before(clean,_space_before(clean,at-2))
        if re.fullmatch(r'[a-z][a-z0-9-]*',identifier)is None:raise ValueError('unsupported Slint instance identifier')
        at=_space_before(clean,at)
    return typ,identifier,at


def _literal_false(clean,at):
    """Recognize only grouped if false immediately before the parsed instance.

    Header custody comes from _instance_header; no operators, properties or
    general condition values are evaluated. All scans follow disjoint local
    tokens under the existing source byte and node/depth bounds.
    """
    def word(at,value):
        start=at-len(value)
        if start<0 or clean[start:at]!=value:return None
        if start and(clean[start-1].isalnum()or clean[start-1]in '_-'):return None
        return _space_before(clean,start)
    if not at or clean[at-1]!=':':return None
    at=_space_before(clean,at-1);groups=0
    while at and clean[at-1]==')':groups+=1;at=_space_before(clean,at-1)
    at=word(at,'false')
    if at is None:return None
    for _ in range(groups):
        if not at or clean[at-1]!='(':return None
        at=_space_before(clean,at-1)
    return word(at,'if')


def _nodes(clean,components):
    nodes=[];stack=[];declarations={match.end()-1:match for match in COMPONENT.finditer(clean)}
    for at,token in enumerate(clean):
        if token=='{':
            if len(nodes)>=MAX_NODES or len(stack)>=MAX_BRACE_DEPTH:raise ValueError('Slint node bound')
            parent=stack[-1] if stack else None
            inherited=nodes[parent]['component'] if parent is not None else None
            declaration=declarations.get(at);literal_false=None
            if declaration:
                name,base=declaration[1],declaration[2]
                if name in components:raise ValueError('duplicate Slint component')
                components[name]=base
                typ=name;component=name;identifier=None
            else:
                typ,identifier,before=_instance_header(clean,at)
                component=inherited
                literal_false=_literal_false(clean,before)if typ is not None else None
            nodes.append({'start':at,'end':None,'parent':parent,'type':typ,'component':component,'identifier':identifier,
                          'dead':literal_false is not None or(parent is not None and nodes[parent]['dead']),
                          'dead_start':literal_false,
                          'declaration':declaration is not None,'exported':declaration is not None and bool(re.search(r'\bexport\s+component\b',declaration[0]))})
            stack.append(len(nodes)-1)
        elif token=='}':
            if not stack:raise ValueError('Slint closing brace')
            nodes[stack.pop()]['end']=at
    if stack:raise ValueError('incomplete Slint braces')
    return nodes


def _owner(nodes,opens,at):
    index=bisect_right(opens,at)-1
    while index>=0:
        node=nodes[index]
        if node['start']<at<node['end'] and node['dead']:return None
        if node['start']<at<node['end'] and node['type'] is not None:return node
        parent=node['parent']
        if parent is None:return None
        index=parent
    return None


def _reachable_components(records,components):
    """Follow actual component instantiation/inheritance from protected roots.

    Children, Tooltip and menu instances keep their containing component's
    custody; their custom definitions and inherited bases become reachable.
    Only literal-false subtrees are pruned; other conditions stay source may-flow.
    """
    composition=defaultdict(set);roots=set();ranges=defaultdict(list);edge_count=0
    def add(source,target):
        nonlocal edge_count
        if target not in composition[source]:
            edge_count+=1
            if edge_count>MAX_EDGES:raise ValueError('Slint composition edge bound')
            composition[source].add(target)
    for path,_,_,nodes in records:
        for node in nodes:
            if node['dead']:continue
            if node['declaration']:
                add(node['type'],components[node['type']])
                if node['parent'] is None and node['exported'] and (path,node['type']) in PRODUCT_ROOTS:
                    roots.add(node['type'])
            elif node['component'] and node['type']:
                add(node['component'],node['type'])
    pending=deque(sorted(roots));reachable=set()
    while pending:
        component=pending.popleft()
        if component in reachable:continue
        reachable.add(component)
        if len(reachable)>MAX_NODES+MAX_EDGES:raise ValueError('Slint composition work bound')
        pending.extend(sorted(composition.get(component,())))
    for path,_,_,nodes in records:
        for node in nodes:
            if node['declaration'] and node['type'] in reachable:
                ranges[path].append((node['start'],node['end']))
    return reachable,{path:sorted(spans) for path,spans in ranges.items()},roots


class Trace:
    """Finite source binding graph and native text endpoints, with no catalog input."""
    def __init__(self,edges,leaves,locations,bindings,active_ranges,components,roots,window_components,instances=None,instance_details=None,dead_ranges=None):
        self.edges=edges;self.leaves=leaves;self.locations=locations;self.bindings=bindings
        self.active_ranges=active_ranges;self.components=components;self.roots=roots
        self.window_components=window_components
        self.instances=instances or {}
        self.instance_details=instance_details or {}
        self.dead_ranges=dead_ranges or {}

    def active_source(self,path,position):
        """Whether one source position belongs to a reached component body."""
        dead=self.dead_ranges.get(path,())
        at=bisect_right(dead,(position,float('inf')))-1
        if at>=0 and dead[at][0]<position<dead[at][1]:return False
        spans=self.active_ranges.get(path,())
        at=bisect_right(spans,(position,float('inf')))-1
        return at>=0 and spans[at][0]<position<spans[at][1]

    def sinks(self,node):
        """Return actual reachable text endpoints; finite visited set handles cycles."""
        node=tuple(node)
        # Component/property entry queries (the runtime sink manifest) expand
        # to concrete instances. Internal edges never pass through a shared
        # component-type/property vertex, so separate instances cannot join.
        initial=[] if node[0].startswith('@') else [(identity,node[1]) for identity,types in self.instances.items() if node[0] in types]
        queue=deque(initial or [node]);seen=set();sinks=[]
        while queue:
            current=queue.popleft()
            if current in seen:continue
            seen.add(current)
            if len(seen)>MAX_EDGES+MAX_NODES:raise ValueError('Slint trace work bound')
            if current in self.leaves:sinks.append(current)
            queue.extend(sorted(self.edges.get(current,())))
        return sorted(sinks)

    def reaches(self,component,prop):
        """Whether one component input has any recognized actual text endpoint."""
        return bool(self.sinks((component,prop)))

    def instance_path(self,identity):
        """Return concrete source custody from root through this instance."""
        path=[];seen=set()
        while identity is not None:
            if identity in seen or len(seen)>MAX_NODES:raise ValueError('Slint instance ancestry bound')
            seen.add(identity);info=self.instance_details[identity]
            path.append({'instance':identity,'owner':info['type'],'path':info['path'],'line':info['line']})
            identity=info['parent']
        return list(reversed(path))


def build(files,inert_source):
    """Trace concrete component instances from the protected product root.

    Every instance has separate properties and lexical identifier custody.
    Inherited definitions share the derived instance; its caller bindings
    override definition defaults. Provided @children keep their caller's root
    and named-ID scope. Only literal-false subtrees are excluded; dynamic
    conditions/repeaters remain bounded source may-flow.
    """
    components={};records=[];dead_ranges={}
    for path,source in sorted(files.items()):
        if not path.endswith('.slint') or path in COPY_SOURCE_PATHS:continue
        clean=inert_source(source);nodes=_nodes(clean,components)
        records.append((path,source,clean,nodes))
        dead_ranges[path]=[(node['dead_start'],node['end'])for node in nodes if node['dead']and(node['parent']is None or not nodes[node['parent']]['dead'])]
    reachable,active_ranges,roots=_reachable_components(records,components)
    window_components=set()
    for component in components:
        ancestry=set();base=component
        while base in components:
            if base in ancestry:raise ValueError('Slint inheritance cycle')
            ancestry.add(base);base=components[base]
        if base=='Window':window_components.add(component)
    definitions={};source_bindings={};children={};node_tables={};line_tables={}
    for path,source,clean,nodes in records:
        node_tables[path]=nodes;line_tables[path]=[at for at,ch in enumerate(source)if ch=='\n']
        opens=[node['start']for node in nodes]
        for index,node in enumerate(nodes):
            children[(path,index)]=[]
            if node['parent'] is not None:children[(path,node['parent'])].append(index)
            node['path']=path;node['index']=index
            if node['declaration']:definitions[node['type']]=(path,node,nodes)
        for found in outside_parameters(clean,PROPERTY.finditer(clean)):
            spans=dead_ranges[path];at=bisect_right(spans,(found.start(),float('inf')))-1
            if at>=0 and spans[at][0]<=found.start()<spans[at][1]:continue
            owner=_owner(nodes,opens,found.start())
            if owner is None:continue
            end=_expression_end(clean,found.end())
            binding={'path':path,'owner':owner['type'],'component':owner['component'],'property':found[1],
                     'start':found.end(),'end':end,'expression':clean[found.end():end],
                     'line':source.count('\n',0,found.start())+1}
            source_bindings.setdefault((path,owner['index']),[]).append(binding)
    edges=defaultdict(set);locations=defaultdict(list);binding_count=0;binding_records=[];edge_count=0
    def add(source,target,location=None):
        nonlocal edge_count
        if target not in edges[source]:
            edge_count+=1
            if edge_count>MAX_EDGES:raise ValueError('Slint binding edge bound')
            edges[source].add(target)
        if location is not None:locations[(source,target)].append(location)
    instances={};effective={};instance_types={};instance_parents={};lineages={}
    def ancestry(typ):
        if typ in lineages:return lineages[typ]
        initial=typ
        result=[]
        while typ is not None:
            if typ in result:raise ValueError('Slint inheritance cycle')
            if len(result)>=MAX_BRACE_DEPTH:raise ValueError('Slint inheritance depth bound')
            result.append(typ);typ=components.get(typ)
        lineages[initial]=tuple(result)
        return lineages[initial]
    def bind(node,identity,scope):
        nonlocal binding_count
        for binding in source_bindings.get((node['path'],node['index']),()):
            binding_count+=1
            if binding_count>MAX_BINDINGS:raise ValueError('Slint binding count')
            target=(identity,binding['property'])
            binding_records.append({key:value for key,value in binding.items() if key not in {'expression','line'}}|{'target':target})
            # The last layer wins: base definition, derived definition, then
            # the caller's concrete instance binding. Dead default bindings
            # cannot establish a copy flow into a replaced property.
            effective[target]=(binding,scope,identity)
    def direct_children(path,node,nodes):
        for index in children[(path,node['index'])]:
            child=nodes[index]
            if child['declaration']or child['dead']:continue
            if child['type'] is None:yield from direct_children(path,child,nodes)
            else:yield child
    def definition(typ,identity,stack):
        for layer in reversed(ancestry(typ)):
            if layer not in definitions:continue
            path,node,nodes=definitions[layer]
            scope={'root':identity,'ids':{}}
            bind(node,identity,scope)
            for child in direct_children(path,node,nodes):instantiate(child,scope,stack,identity)
    def instantiate(node,scope,stack,parent):
        typ=node['type']
        if typ in stack:raise ValueError('recursive Slint component composition')
        if len(stack)>=MAX_BRACE_DEPTH:raise ValueError('Slint composition depth bound')
        if len(instances)>=MAX_NODES:raise ValueError('Slint instance bound')
        identity='@instance'+str(len(instances));instances[identity]=node
        instance_parents[identity]=parent
        instance_types[identity]=ancestry(typ)
        if node['identifier']:scope['ids'][node['identifier']]=identity
        definition(typ,identity,(*stack,typ))
        bind(node,identity,scope)
        # Caller-provided children retain caller lexical root/ID custody even
        # when a custom component places them in an inherited @children slot.
        path=node['path'];nodes=node_tables[path]
        for child in direct_children(path,node,nodes):instantiate(child,scope,stack,identity)
        return identity
    for path,source,clean,nodes in records:
        for node in nodes:
            if node['declaration'] and node['parent'] is None and (path,node['type']) in PRODUCT_ROOTS:
                if len(instances)>=MAX_NODES:raise ValueError('Slint instance bound')
                identity='@instance'+str(len(instances));instances[identity]=node
                instance_parents[identity]=None
                instance_types[identity]=ancestry(node['type'])
                definition(node['type'],identity,(node['type'],))
    for target,(binding,scope,identity) in effective.items():
        expression=binding['expression'];location={'path':binding['path'],'line':binding['line']}
        for reference in COPY.finditer(expression):add(('@copy',reference[1]+reference[2]),target,location)
        for reference in LOCAL.finditer(expression):
            name,prop=reference[1],reference[2]
            origin=scope['root'] if name=='root' else identity if name=='self' else scope['ids'].get(name)
            if origin:add((origin,prop),target,location)
    all_nodes=set(edges)
    all_nodes.update(binding['target']for binding in binding_records)
    for targets in edges.values():all_nodes.update(targets)
    leaves=set()
    for identity,prop in all_nodes:
        if identity=='@copy':continue
        if prop in {'accessible-label','accessible-description'}:leaves.add((identity,prop));continue
        if any(prop in NATIVE_TEXT.get(base,set())for base in instance_types.get(identity,())):leaves.add((identity,prop))
    details={identity:{'type':node['type'],'ancestry':instance_types[identity],
                       'parent':instance_parents[identity],
                       'start':node['start'],'component':node['component'],
                       'path':node['path'],'line':bisect_right(line_tables[node['path']],node['start'])+1,
                       'bindings':{}}
             for identity,node in instances.items()}
    for (identity,prop),(binding,scope,_) in effective.items():details[identity]['bindings'][prop]=binding|{'root_instance':scope['root']}
    return Trace(edges,leaves,locations,binding_records,active_ranges,reachable,roots,window_components,instance_types,details,dead_ranges)
