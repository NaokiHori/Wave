To see the energy balance, we multiply the equation of motion by :math:`\rho \vel{}{}` and volume-integrate it:

.. math::

    \myint{0}{L}{
        \rho
        \vel{}{}
        \pder{\vel{}{}}{t}
    }{x}
    -
    \myint{0}{L}{
        \rho
        \vel{}{}
        c^2
        \pder{}{x}
        \pder{\pos{}{}}{x}
    }{x}
    -
    \myint{0}{L}{
        \rho
        \vel{}{}
        \nu
        \pder{}{x}
        \pder{\vel{}{}}{x}
    }{x}
    -
    \myint{0}{L}{
        \rho
        \vel{}{}
        a
    }{x}
    =
    0,

where :math:`L` is the length of the domain.

With some algebra, we see that the first term is equal to

.. math::

    \pder{}{t}
    \left(
        \myint{0}{L}{
            \frac{1}{2}
            \rho
            \vel{}{}^2
        }{x}
    \right)
    =
    \tder{K}{t},

which describes the change of the total kinetic energy :math:`K` in time.

To reformulate the second term, we adopt the integration by parts:

.. math::

    -
    \myint{0}{L}{
        \rho
        c^2
        \vel{}{}
        \pder{}{x}
        \pder{\pos{}{}}{x}
    }{x}
    =
    -
    \left[
        \rho
        c^2
        \vel{}{}
        \pder{\pos{}{}}{x}
    \right]_{0}^{L}
    +
    \myint{0}{L}{
        \rho
        c^2
        \pder{\vel{}{}}{x}
        \pder{\pos{}{}}{x}
    }{x},

where the former term denotes the boundary contribution, while the latter term is attributed to the change of the total potential energy :math:`U` in time as it can be written as

.. math::

    \myint{0}{L}{
        \rho
        c^2
        \pder{}{t}
        \left(
            \pder{\pos{}{}}{x}
        \right)
        \pder{\pos{}{}}{x}
    }{x}
    =
    \pder{}{t}
    \left(
        \myint{0}{L}{
            \frac{1}{2}
            \rho
            c^2
            \pder{\pos{}{}}{x}
            \pder{\pos{}{}}{x}
        }{x}
    \right)
    =
    \tder{U}{t}.

Similarly, the third term can be reformulated by adopting the integration by parts:

.. math::

    -
    \myint{0}{L}{
        \rho
        \nu
        \vel{}{}
        \pder{}{x}
        \pder{\vel{}{}}{x}
    }{x}
    =
    -
    \left[
        \rho
        \nu
        \vel{}{}
        \pder{\vel{}{}}{x}
    \right]_{0}^{L}
    +
    \myint{0}{L}{
        \rho
        \nu
        \pder{\vel{}{}}{x}
        \pder{\vel{}{}}{x}
    }{x},

where the former term is again the boundary contribution whereas the latter denotes the energy dissipation.

The fourth term simply denotes the energy input due to the external factor, and we remain it as it is.

In summary, we obtain

.. math::

    \tder{E}{t}
    =
    \left[
        \rho
        c^2
        \vel{}{}
        \pder{\pos{}{}}{x}
    \right]_{0}^{L}
    +
    \left[
        \rho
        \nu
        \vel{}{}
        \pder{\vel{}{}}{x}
    \right]_{0}^{L}
    -
    \myint{0}{L}{
        \rho
        \nu
        \pder{\vel{}{}}{x}
        \pder{\vel{}{}}{x}
    }{x}
    +
    \myint{0}{L}{
        \rho
        \vel{}{}
        a
    }{x},

where :math:`E \equiv K + U` is the total energy of the system.

