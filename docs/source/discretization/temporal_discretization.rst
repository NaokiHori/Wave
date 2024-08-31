We focus on a temporal discretization which conserves the total energy up to rounding error in the absence of external factors (dissipation, external forcing).

To start, the relation between the displacement and the velocity is discretized by means of the Crank-Nicolson scheme:

.. math::

    \dif{\kpos{k}{}}{t}
    =
    \ave{\kvel{k}{}}.

Here we introduce two symbols for brevity:

.. math::

    \delta Q
    &
    \equiv
    Q^{n + 1}
    -
    Q^{n    },

    \ave{Q}
    &
    \equiv
    \frac{1}{2}
    Q^{n + 1}
    +
    \frac{1}{2}
    Q^{n    }.

To discretize the equation of motion:

.. math::

    \tder{\kvel{k}{}}{t}
    +
    N_k
    \kvel{k}{}
    =
    -
    C_k
    \kpos{k}{}
    +
    A_k,

we multiply

.. math::

    \intfactor{t}
    \equiv
    \exp \left( N_k t \right),

which is known as integrating factor, and integrate the equation in time to yield

.. math::

    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \left\{
            \intfactor{t}
            \tder{Z_k}{t}
            +
            \intfactor{t}
            N_k
            \kvel{k}{}
        \right\}
    }{
        t
    }
    =
    -
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t}
        C_k
        \kpos{k}{}
    }{
        t
    }
    +
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t}
        A_k
    }{
        t
    }.

Due to

.. math::

    \intfactor{t}
    N_k
    =
    \tder{}{t}
    \left\{
        \intfactor{t}
    \right\},

the left-hand-side term leads to

.. math::

    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \left[
            \intfactor{t}
            \tder{Z_k}{t}
            +
            \tder{}{t}
            \left\{
                \intfactor{t}
            \right\}
            \kvel{k}{}
        \right]
    }{
        t
    }
    =
    \left[
        \intfactor{t}
        Z_k
    \right]_{t^n}^{t^{n+1}},

and thus we obtain

.. math::

    \intfactor{t^{n + 1}}
    Z_k^{n + 1}
    -
    \intfactor{t^{n    }}
    Z_k^{n    }
    =
    -
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t}
        C_k
        \kpos{k}{}
    }{
        t
    }
    +
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t}
        A_k
    }{
        t
    }.

The first term in the right-hand side adopts the Crank-Nicolson scheme to achieve :ref:`the discrete energy conservation <appendix_discrete_energy_conservation>`:

.. math::

    -
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t}
        C_k
        \kpos{k}{}
    }{
        t
    }
    =
    -
    C_k
    \frac{1}{2}
    \left\{
        \intfactor{t^{n + 1}}
        \kpos{k}{n + 1}
        +
        \intfactor{t^{n    }}
        \kpos{k}{n    }
    \right\}
    \Delta t.

By dividing the relation by :math:`\intfactor{t^{n}}`, we obtain a conclusive scheme:

.. math::

    \intfactor{\Delta t}
    Z_k^{n + 1}
    -
    Z_k^{n    }
    =
    -
    C_k
    \frac{1}{2}
    \left\{
        \intfactor{\Delta t}
        \kpos{k}{n + 1}
        +
        \kpos{k}{n    }
    \right\}
    \Delta t
    +
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t - t^n}
        A_k
    }{
        t
    }.

The discretization of the external forcing term is arbitrary, and one possible way is elaborated in :ref:`elsewhere <appendix_external_forcing>`.

In summary, a set of discrete equations of interest is

.. math::

    \begin{pmatrix}
        1 & - \tau \\
        C_k \tau \intfactor{\Delta t} & \intfactor{\Delta t}
    \end{pmatrix}
    \begin{pmatrix}
        \kpos{k}{n + 1} \\
        \kvel{k}{n + 1}
    \end{pmatrix}
    =
    \begin{pmatrix}
        1 & \tau \\
        - C_k \tau & 1
    \end{pmatrix}
    \begin{pmatrix}
        \kpos{k}{n    } \\
        \kvel{k}{n    }
    \end{pmatrix}
    +
    \begin{pmatrix}
        0 \\
        \myint{
            t^n
        }{
            t^{n+1}
        }{
            \intfactor{t - t^n}
            A_k
        }{
            t
        }
    \end{pmatrix}.

