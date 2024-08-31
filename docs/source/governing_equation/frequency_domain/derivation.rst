For simplicity, we impose Dirichlet boundary conditions with respect to the displacement:

.. math::

    \pos{}{}
    \equiv
    0

and to the velocity:

.. math::

    \vel{}{}
    \equiv
    0

in this project for now, indicating that the boundaries are *pinned*.

This choice allows us to expand the fields using sinusoidal functions:

.. math::

    \pos{}{} \left( x, t \right)
    &
    =
    \mysum{l}{0}{\infty}
    \kpos{l}{} \left( t \right)
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right),

    \vel{}{} \left( x, t \right)
    &
    =
    \mysum{l}{0}{\infty}
    \kvel{l}{} \left( t \right)
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right).

Although :math:`\kpos{l}{}` and :math:`\kvel{k}{}` are both function in time, they are not explicitly given later for brevity.

Assigning them to the governing equations yields

.. math::

    \mysum{l}{0}{\infty}
    \tder{\kpos{l}{}}{t}
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)
    =
    &
    \mysum{l}{0}{\infty}
    \kvel{l}{}
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right),

    \mysum{l}{0}{\infty}
    \tder{\kvel{l}{}}{t}
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)
    =
    &
    -
    \mysum{l}{0}{\infty}
    c^2
    \left( \wavenumber{\left( l + 1 \right)}{}{L} \right)^2
    \kpos{l}{}
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)

    &
    -
    \mysum{l}{0}{\infty}
    \nu
    \left( \wavenumber{\left( l + 1 \right)}{}{L} \right)^2
    \kvel{l}{}
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)

    &
    +
    \mysum{l}{0}{\infty}
    A_{l}
    \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right),

Due to the orthogonality:

.. math::

    &
    \myint{0}{L}{
        q \left( x, t \right)
        \sin \left( \wavenumber{\left( k + 1 \right)}{x}{L} \right)
    }{x}

    =
    &
    \myint{0}{L}{
        \mysum{l}{0}{\infty}
        Q_{l} \left( t \right)
        \sin \left( \wavenumber{\left( l + 1 \right)}{x}{L} \right)
        \sin \left( \wavenumber{\left( k + 1 \right)}{x}{L} \right)
    }{x}

    =
    &
    \myint{0}{L}{
        \mysum{l}{0}{\infty}
        Q_{l} \left( t \right)
        \frac{1}{2}
        \cos \left( \wavenumber{\left( l - k \right)}{x}{L} \right)
    }{x}
    -
    \myint{0}{L}{
        \mysum{l}{0}{\infty}
        Q_{l} \left( t \right)
        \frac{1}{2}
        \cos \left( \wavenumber{\left( l + k + 2 \right)}{x}{L} \right)
    }{x}

    =
    &
    \frac{L}{2}
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
    \left( \wavenumber{\left( k + 1 \right)}{}{L} \right)^2,

    N_k
    &
    \equiv
    \nu
    \left( \wavenumber{\left( k + 1 \right)}{}{L} \right)^2.

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
    \left( \wavenumber{\left( k_x + 1 \right)}{}{L_x} \right)^2
    +
    c^2
    \left( \wavenumber{\left( k_y + 1 \right)}{}{L_y} \right)^2,

    N_{k_x,k_y}
    &
    \equiv
    \nu
    \left( \wavenumber{\left( k_x + 1 \right)}{}{L_x} \right)^2
    +
    \nu
    \left( \wavenumber{\left( k_y + 1 \right)}{}{L_y} \right)^2.

Higher dimensional relations can be obtained in the same manner.

