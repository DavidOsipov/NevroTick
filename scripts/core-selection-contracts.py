"""Literal external Rust consumers for S1.3 fixture construction/output contracts.

GPL-2.0-only, David Osipov with AI assistance, 30 October 2026.
These are compiler/executable input fixtures, not observed acceptance results.
The existing core verifier owns source binding, actual dependency artifacts,
intended compiler diagnostics, process custody and pre-effect reservations.
"""
from __future__ import annotations


def _negative(imports, setup, expression, code, tokens):
    """Pin the actual attempted operation's line, not an unrelated compiler error."""
    prefix = imports + "fn main(){\n" + setup
    return {"source": prefix + expression + "\n}\n", "code": code,
            "tokens": tokens, "line": len(prefix.splitlines()) + 1}


def contract_examples():
    """Return one legal control and 26 safe-construction/privacy bypass attempts.

    No serde graph or unsafe forgery is introduced to manufacture a rejection.
    The legal caller charges six interval, three minimum, four batch and two
    evaluator attempts before entering their public owners. Compilation failures
    have no product runtime calls; successful compile is no privacy verdict.
    """
    imports = ('use time_core::{FixtureInterval,FixtureMinimum,FixtureBatch,FixtureWitness,'
               'FixtureSelection,FixtureOutcomeKind,IntervalError,MinimumError,BatchError,'
               'SelectionError,SelectionInvariantStage,select_fixture};\n'
               'use std::num::NonZeroU64;\n')
    legal = imports + r'''
struct Calls{intervals:u8,minima:u8,batches:u8,selections:u8}
impl Calls{
 fn interval(&mut self,label:u64,lo:i64,hi:i64)->Result<FixtureInterval,IntervalError>{
  self.intervals=self.intervals.checked_add(1).unwrap();FixtureInterval::new(label,lo,hi)
 }
 fn minimum(&mut self,value:usize)->Result<FixtureMinimum,MinimumError>{
  self.minima=self.minima.checked_add(1).unwrap();FixtureMinimum::new(value)
 }
 fn batch(&mut self,values:&[FixtureInterval])->Result<FixtureBatch,BatchError>{
  self.batches=self.batches.checked_add(1).unwrap();FixtureBatch::new(values)
 }
 fn select(&mut self,batch:&FixtureBatch,minimum:FixtureMinimum)->FixtureSelection{
  self.selections=self.selections.checked_add(1).unwrap();select_fixture(batch,minimum).unwrap()
 }
}
impl Drop for Calls{fn drop(&mut self){
 eprintln!("CORE_SELECTION_EXTERNAL_PROFILE intervals={} minima={} batches={} selections={}",
  self.intervals,self.minima,self.batches,self.selections);
}}
fn main(){
 let mut calls=Calls{intervals:0,minima:0,batches:0,selections:0};
 let a=calls.interval(3,-4,5).unwrap();
 let b=calls.interval(1,-2,3).unwrap();
 let c=calls.interval(2,0,6).unwrap();
 let private_values=calls.interval(u64::MAX,i64::MIN,i64::MAX).unwrap();
 let zero=calls.interval(0,123456789,-987654321).unwrap_err();
 assert_eq!(zero,IntervalError::ZeroLabel);
 let reversed=calls.interval(7,i64::MAX,i64::MIN).unwrap_err();
 assert_eq!(reversed,IntervalError::ReversedBounds);
 let low=calls.minimum(0).unwrap_err();
 let high=calls.minimum(17).unwrap_err();
 assert_eq!((low,high),(MinimumError::OutOfRange,MinimumError::OutOfRange));
 let minimum=calls.minimum(3).unwrap();
 let batch=calls.batch(&[a,b,c]).unwrap();
 let before:Vec<_>=batch.iter().map(|v|(v.label().get(),v.lower(),v.upper())).collect();
 let duplicate=calls.batch(&[a,b,a]).unwrap_err();
 assert_eq!(duplicate,BatchError::DuplicateLabel{later_index:2});
 let capacity=calls.batch(&[a;17]).unwrap_err();
 assert_eq!(capacity,BatchError::CapacityExceeded);
 let empty=calls.batch(&[]).unwrap();
 let result=calls.select(&batch,minimum);
 assert_eq!((result.population(),result.minimum(),result.maximum_depth()),(3,3,3));
 assert!(result.strict_majority()&&result.minimum_met());
 assert_eq!(result.outcome(),FixtureOutcomeKind::WitnessedFixture);
 assert_eq!(result.component_count(),None);
 let witness=result.witnessed().unwrap();
 assert_eq!((witness.component_lower(),witness.component_upper(),witness.witness()),(0,3,0));
 assert_eq!((witness.intersection_lower(),witness.intersection_upper()),(0,3));
 assert_eq!(witness.supporters().map(NonZeroU64::get).collect::<Vec<_>>(),[1,2,3]);
 let no_data=calls.select(&empty,minimum);
 assert_eq!((no_data.population(),no_data.minimum(),no_data.maximum_depth()),(0,3,0));
 assert_eq!(no_data.outcome(),FixtureOutcomeKind::Empty);
 assert!(!no_data.strict_majority()&&!no_data.minimum_met());
 assert!(no_data.witnessed().is_none());
 assert_eq!(batch.iter().map(|v|(v.label().get(),v.lower(),v.upper())).collect::<Vec<_>>(),before);
 assert_eq!(minimum.get(),3);
 for text in [format!("{private_values:?}"),format!("{minimum:?}"),format!("{batch:?}"),
              format!("{witness:?}"),format!("{result:?}")]{
  assert!(!text.is_empty()&&text.len()<128);
  assert!(!text.chars().any(|c|c.is_ascii_digit()));
 }
 for text in [zero.to_string(),reversed.to_string(),low.to_string(),high.to_string(),
              duplicate.to_string(),capacity.to_string(),format!("{a:?}")]{
  assert!(!text.is_empty()&&text.len()<128);
  for raw in ["123456789","987654321","9223372036854775807","9223372036854775808"]{
   assert!(!text.contains(raw));
  }
 }
 assert!(std::error::Error::source(&zero).is_none());
 assert!(std::error::Error::source(&reversed).is_none());
 assert!(std::error::Error::source(&low).is_none());
 assert!(std::error::Error::source(&duplicate).is_none());
 assert!(std::error::Error::source(&capacity).is_none());
 for stage in [SelectionInvariantStage::Slot,SelectionInvariantStage::Component,
               SelectionInvariantStage::Support,SelectionInvariantStage::Intersection]{
  let leaf=SelectionError::Invariant{stage};
  assert!(leaf.to_string().len()<128);
  assert!(std::error::Error::source(&leaf).is_none());
 }
 assert_eq!((calls.intervals,calls.minima,calls.batches,calls.selections),(6,3,4,2));
 println!("legal-core-contract");
}
'''
    cases = {"legal_selection_construction_and_output": {
        "source": legal, "code": None,
        "calls": {"direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0},
        "selection_calls": {"intervals": 6, "minima": 3, "batches": 4, "selections": 2}}}
    literals = {
        "FixtureInterval": 'label:NonZeroU64::MIN,bounds:(0,1)',
        "FixtureMinimum": 'value:1',
        "FixtureBatch": 'values:[None;16],len:0',
        "FixtureWitness": 'component:(0,1),witness:0,intersection:(0,1),labels:[NonZeroU64::MIN;16],label_count:1',
        "FixtureSelection": 'population:1,minimum:1,maximum_depth:1,strict_majority:true,minimum_met:true,outcome:FixtureOutcomeKind::WitnessedFixture,component_count:None,witnessed:None',
    }
    from_values = {"FixtureInterval": ('(u64,i64,i64)', '(1,0,1)'),
                   "FixtureMinimum": ('usize', '0'),
                   "FixtureBatch": ('&[FixtureInterval]', '&[]'),
                   "FixtureWitness": ('()', '()'), "FixtureSelection": ('()', '()')}
    for role, fields in literals.items():
        name = role.removeprefix('Fixture').lower()
        cases['selection_' + name + '_default'] = _negative(
            imports, '', ' let _: '+role+'=Default::default();', 'E0277', [role, 'Default'])
        cases['selection_' + name + '_literal'] = _negative(
            imports, '', ' let _='+role+'{'+fields+'};', 'E0451', [role, 'private'])
        raw_type, raw_value = from_values[role]
        cases['selection_' + name + '_from'] = _negative(
            imports, '', ' let _=<'+role+' as From<'+raw_type+'>>::from('+raw_value+');',
            'E0277', [role, 'From'])
    setups = {
        'interval': ' let mut value=FixtureInterval::new(1,0,1).unwrap();\n',
        'minimum': ' let mut value=FixtureMinimum::new(1).unwrap();\n',
        'batch': ' let mut value=FixtureBatch::new(&[]).unwrap();\n',
        'result': (' let batch=FixtureBatch::new(&[]).unwrap();\n'
                   ' let mut value=select_fixture(&batch,FixtureMinimum::new(1).unwrap()).unwrap();\n'),
    }
    mutations = [('interval','label','NonZeroU64::MIN'),('interval','bounds','(1,0)'),
                 ('minimum','value','0'),('batch','values','[None;16]'),('batch','len','17'),
                 ('result','maximum_depth','0'),('result','witnessed','None')]
    for role, field, value in mutations:
        cases['selection_' + role + '_mutation_' + field] = _negative(
            imports, setups[role], ' value.'+field+'='+value+';', 'E0616', [field, 'private'])
    cases['selection_private_module'] = _negative(
        '', '', ' use time_core::selection;', 'E0603', ['selection', 'private'])
    cases['selection_raw_batch_input'] = _negative(
        imports, '', ' let _=FixtureBatch::new(&[0_u64]);', 'E0308', ['FixtureInterval', 'u64'])
    cases['selection_raw_evaluator_batch'] = _negative(
        imports, '', ' let _=select_fixture(&[],FixtureMinimum::new(1).unwrap());',
        'E0308', ['FixtureBatch'])
    cases['selection_raw_evaluator_minimum'] = _negative(
        imports, ' let batch=FixtureBatch::new(&[]).unwrap();\n',
        ' let _=select_fixture(&batch,0_usize);', 'E0308', ['FixtureMinimum', 'usize'])
    return cases
