In this project, we assume the domain is periodic in all directions for simplicity.
This choice allows us to expand the fields using trigonometric functions:

.. math::

    \pos{}{} \left( x, t \right)
    &
    =
    \mysum{l}{-\infty}{\infty}
    \kpos{l}{} \left( t \right)
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right),

    \vel{}{} \left( x, t \right)
    &
    =
    \mysum{l}{-\infty}{\infty}
    \kvel{l}{} \left( t \right)
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right).

Although :math:`\kpos{l}{}` and :math:`\kvel{k}{}` are both function in time, they are not explicitly indicated later for brevity.

Assigning them to the governing equations yields

.. math::

    \mysum{l}{-\infty}{\infty}
    \tder{\kpos{l}{}}{t}
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)
    =
    &
    \mysum{l}{-\infty}{\infty}
    \kvel{l}{}
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right),

    \mysum{l}{-\infty}{\infty}
    \tder{\kvel{l}{}}{t}
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)
    =
    &
    -
    \mysum{l}{-\infty}{\infty}
    c^2
    \left( \wavenumber{2 \pi}{l}{L} \right)^2
    \kpos{l}{}
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)

    &
    -
    \mysum{l}{-\infty}{\infty}
    \nu
    \left( \wavenumber{2 \pi}{l}{L} \right)^2
    \kvel{l}{}
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)

    &
    +
    \mysum{l}{-\infty}{\infty}
    A_l
    \exp \left( \wavenumber{2 \pi}{l x}{L} I \right).

Due to the orthogonality:

.. math::

    &
    \myint{0}{L}{
        q \left( x, t \right)
        \exp \left( \wavenumber{- 2 \pi}{k x}{L} I \right)
    }{x}

    =
    &
    \myint{0}{L}{
        \mysum{l}{-\infty}{\infty}
        Q_l \left( t \right)
        \exp \left( \wavenumber{2 \pi}{l x}{L} I \right)
        \exp \left( \wavenumber{- 2 \pi}{k x}{L} I \right)
    }{x}

    =
    &
    \myint{0}{L}{
        \mysum{l}{-\infty}{\infty}
        Q_l \left( t \right)
        \exp \left( \wavenumber{2 \pi}{\left( l - k \right) x}{L} I \right)
    }{x}

    =
    &
    L
    Q_{k} \left( t \right),

we obtain

.. math::

    \tder{\kpos{k}{}}{t}
    =
    &
    \kvel{k}{},

    \tder{\kvel{k}{}}{t}
    =
    &
    -
    C_k
    \kpos{k}{}
    -
    N_k
    \kvel{k}{}
    +
    A_k,

where

.. math::

    C_k
    &
    \equiv
    c^2
    \left( \wavenumber{2 \pi}{k}{L} \right)^2,

    N_k
    &
    \equiv
    \nu
    \left( \wavenumber{2 \pi}{k}{L} \right)^2.

For two-dimensional cases, we have

.. math::

    \tder{\kpos{k_x,k_y}{}}{t}
    =
    &
    \kvel{k_x,k_y}{},

    \tder{\kvel{k_x,k_y}{}}{t}
    =
    &
    -
    C_{k_x,k_y}
    \kpos{k_x,k_y}{}
    -
    N_{k_x,k_y}
    \kvel{k_x,k_y}{}
    +
    A_{k_x,k_y},

where the following symbols are defined for notational simplicity:

.. math::

    C_{k_x,k_y}
    &
    \equiv
    c^2
    \left( \wavenumber{2 \pi}{k_x}{L_x} \right)^2
    +
    c^2
    \left( \wavenumber{2 \pi}{k_y}{L_y} \right)^2,

    N_{k_x,k_y}
    &
    \equiv
    \nu
    \left( \wavenumber{2 \pi}{k_x}{L_x} \right)^2
    +
    \nu
    \left( \wavenumber{2 \pi}{k_y}{L_y} \right)^2.

Higher dimensional relations can be obtained in the same manner.

