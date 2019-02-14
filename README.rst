fast-floats
===========

Please read the `API documentation on docs.rs`__

__ https://docs.rs/fast-floats/

|build_status|_ |crates|_

.. |build_status| image:: https://travis-ci.org/bluss/fast-floats.svg?branch=master
.. _build_status: https://travis-ci.org/bluss/fast-floats

.. |crates| image:: http://meritbadge.herokuapp.com/fast-floats
.. _crates: https://crates.io/crates/fast-floats


Recent Changes
--------------

- 0.3.0

  - Fixed horrible, horrible bug that made none of the assignment operators work except for ``+=``.

- 0.2.0

  - Made ``Fast<T>`` ``#[repr(transparent)]``.
  - Updated optional dependency ``num-traits`` from 0.1.40 -> 0.2.6.
  - Implemented ``min`` and ``max`` methods.

- 0.1.3

  - Implemented more support for standard ``f32`` and ``f64`` methods like ``floor``, ``cos``, ``ln``, etc.

- 0.1.2

  - Implemented ``Neg`` trait.

- 0.1.1

  - Added mixed operations (``Fast<f64> + f64`` etc.).

- 0.1.0

  - Initial release.


License
=======

Dual-licensed to be compatible with the Rust project.

Licensed under the Apache License, Version 2.0
http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
http://opensource.org/licenses/MIT, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
