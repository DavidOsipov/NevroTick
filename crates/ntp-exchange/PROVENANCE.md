# Exchange association source provenance

This is NevroTick's S1.1d pure, single-pending-request, explicit unauthenticated
48-byte header fixture. The retained source context is Chrony **4.9**, commit
`8df4f1263e206585e6c4e61352d042c2dd12916a`. No moving revision, clean-room claim or
language-translation licence exemption is used.

| Pinned source context | Immutable SHA-256 | Selected context |
| --- | --- | --- |
| [ntp_core.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_core.c) | `ca206410bddc03c56996bba2bddbca2ab01029f9c5b619d4a55025f37d8c4ac0` | parse_packet (1548), process_response (1948), NCR_ProcessRxKnown (2531) |
| [ntp_sources.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_sources.c) | `c272f644d8734b6087d8f8d008cb6a9c481a480946dde229d82e2c451fdb2e1f` | NSR_ProcessRx (1288), exact address and port lookup |
| [ntp_io.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_io.c) | `538a626b6c9b512539aba2b61360d6fbf99204cfb8d657f907b489be1d610a75` | process_message (426), transport receipt facts and bounded packet dispatch |

The [source ledger](../../tasks/s0.1-chrony-ledger.json) retains the source-review
history. The [S1.1d task](../../tasks/s1.1-exchange-association.md) owns the reviewed
contract, modification history and candidate-specific execution evidence. Initial
source preparation provides checked input roles and rejecting transition stubs;
it does not implement or qualify response handling. Later implementation and
verification must be recorded there before making behavioral claims.

Local Rust code is authored for the explicit task contract, retaining the selected
upstream provenance and notices. It uses the independently qualified `ntp-wire`
framing/header and `time-domain` exact numerical owners. Saved real local T1 is
separate from the opaque echoed wire token. Current typed binding, generation,
monotonic timing and once-only completion checks replace implicit context. Explicit
numeric era windows carry no anchor trust. Exact signed raw offset/RTT results
provide neither authenticated time nor statistical sample admission.

Deliberate control-path deviation: pinned `ntp_core.c` lines 2029–2087 recognize
RATE only under their duplicate, origin, authentication, unsynchronized-leap and
stratum-zero conditions; lines 2326–2335 set `valid_rx` only for `valid_packet`,
which includes nonzero origin/receive/transmit timestamps. S1.1d instead returns
one opaque terminal control event for every fully correlated stratum-zero header,
including every leap value and zero peer timestamps. It does not unfold control
peer timestamps or perform time arithmetic. This is a specified fixture behavior,
not a claim of matching Chrony's complete control behavior. No RATE backoff,
scheduler, demobilization or other code-dependent action is ported.

Omitted: Unix socket/ancillary/timestamp machinery, source lookup tables, DNS,
network I/O, interleaved/symmetric operation, extensions, authentication/NTS,
statistical eligibility, native clocks/services, IPC and correction. Endpoint and
mapping facts are caller assertions at this pure boundary; later transport must
prove their capture and authority. Token equality proves no entropy, freshness or
authentication. Reusing a token on the same binding can make an old wire reply
indistinguishable; off-wire generations do not repair that limitation.

The [eighteen independent vectors](../../tasks/s1.1-exchange-vectors.json), literal
integer expectations and independent lifecycle model supply oracles separate from
production bodies and upstream output. Upstream reuse supplies no presumption of
correctness, security or resource safety.

Original selected `ntp_core.c` header, retained verbatim:

```text
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2009-2024
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  Core NTP protocol engine
  */
```

Original selected `ntp_sources.c` header, retained verbatim:

```text
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2012, 2014, 2016, 2020-2024
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  Functions which manage the pool of NTP sources that we are currently
  a client of or peering with.

  */
```

Original selected `ntp_io.c` header, retained verbatim:

```text
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Timo Teras  2009
 * Copyright (C) Miroslav Lichvar  2009, 2013-2016, 2018-2021
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  This file deals with the IO aspects of reading and writing NTP packets
  */
```

The selected file-specific grant is **GPL-2.0-only**, without an "or later" grant.
Retain original notices, terms/disclaimer, modification provenance and applicable
corresponding-source/build obligations. This record is not a legal-clearance claim.
The full GPL version 2 text is retained verbatim in [COPYING](COPYING), SHA-256
`ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6`.
