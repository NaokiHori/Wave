The kinetic and potential energies using the variables in frequency domain lead to

.. math::

    K
    &
    \equiv
    \myint{0}{L}{
        \frac{1}{2}
        \rho
        \vel{}{}^2
    }{x}

    &
    =
    \myint{0}{L}{
        \frac{1}{2}
        \rho
        \left(
            \mysum{k}{0}{\infty}
            \kvel{k}{}
            \sin \left( \wavenumber{\left( k + 1 \right)}{x}{L} \right)
        \right)
        \left(
            \mysum{l}{0}{\infty}
            \kvel{l}{}
            \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)
        \right)
    }{x}

    &
    =
    \mysum{k}{0}{\infty}
    \frac{L}{2}
    \frac{1}{2}
    \rho
    \left( \kvel{k}{} \right)^2,

    U
    &
    \equiv
    \myint{0}{L}{
        \frac{1}{2}
        \rho
        c^2
        \pder{\pos{}{}}{x}
        \pder{\pos{}{}}{x}
    }{x}

    &
    =
    \myint{0}{L}{
        \frac{1}{2}
        \rho
        c^2
        \left(
            \mysum{k}{0}{\infty}
            \kpos{k}{}
            \wavenumber{\left( k + 1 \right)}{}{L}
            \cos \left( \wavenumber{\left( k + 1 \right)}{x}{L} \right)
        \right)
        \left(
            \mysum{l}{0}{\infty}
            \kpos{l}{}
            \wavenumber{\left( l + 1 \right)}{}{L}
            \cos \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)
        \right)
    }{x}

    &
    =
    \mysum{k}{0}{\infty}
    \frac{L}{2}
    \frac{1}{2}
    \rho
    C_{k}
    \left( \kpos{k}{} \right)^2,

respectively.

Higher dimensional cases can be derived in the same manner:

.. math::

    K
    &
    =
    \mysum{k_y}{0}{\infty}
    \mysum{k_x}{0}{\infty}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    \left( \kvel{k_x, k_y}{} \right)^2,

    U
    &
    =
    \mysum{k_y}{0}{\infty}
    \mysum{k_x}{0}{\infty}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    C_{k_x, k_y}
    \left( \kpos{k_x, k_y}{} \right)^2.

