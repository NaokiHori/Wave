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
            \mysum{k}{-\infty}{\infty}
            \kvel{k}{}
            \exp \left( \wavenumber{2 \pi}{k x}{L} I \right)
        \right)
        \left(
            \mysum{l}{-\infty}{\infty}
            \kvel{l}{}
            \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)
        \right)
    }{x}

    &
    =
    \mysum{k}{-\infty}{\infty}
    \frac{1}{2}
    \rho
    L
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
            \mysum{k}{-\infty}{\infty}
            \kpos{k}{}
            \wavenumber{2 \pi}{k}{L} I
            \exp \left( \wavenumber{2 \pi}{k x}{L} I \right)
        \right)
        \left(
            \mysum{l}{-\infty}{\infty}
            \kpos{l}{}
            \wavenumber{2 \pi}{l}{L} I
            \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)
        \right)
    }{x}

    &
    =
    \mysum{k}{-\infty}{\infty}
    \frac{1}{2}
    \rho
    L
    C_k
    \left( \kpos{k}{} \right)^2,

respectively.

Higher dimensional cases can be derived in the same manner:

.. math::

    K
    &
    =
    \mysum{k_y}{-\infty}{\infty}
    \mysum{k_x}{-\infty}{\infty}
    L_x
    L_y
    \frac{1}{2}
    \rho
    \left( \kvel{k_x, k_y}{} \right)^2,

    U
    &
    =
    \mysum{k_y}{-\infty}{\infty}
    \mysum{k_x}{-\infty}{\infty}
    L_x
    L_y
    \frac{1}{2}
    \rho
    C_{k_x, k_y}
    \left( \kpos{k_x, k_y}{} \right)^2.

