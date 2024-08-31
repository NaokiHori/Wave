##############
Discretization
##############

**********************
Spatial discretization
**********************

Since we adopt expansions using sinusoidal functions, the spatial discretization is achieved in the spectral space.
Instead of the superposition of infinite number of modes, we truncate it at :math:`N_x`; namely

.. math::

    \mysum{k_y}{\infty}
    \mysum{k_x}{\infty}
    Q_{k_x,k_y}
    \approx
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    Q_{k_x,k_y}.

***********************
Temporal discretization
***********************

We focus on a temporal discretization which conserves the total energy up to rounding error in the absence of external factors (dissipation, external forcing).

To start, the relation between the displacement and the velocity is discretized by means of the Crank-Nicolson scheme:

.. math::

    \dif{\kpos{k_x,k_y}{}}{t}
    =
    \ave{\kvel{k_x,k_y}{}}.

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

The equation of motion as

.. math::

    \frac{d\kvel{k_x,k_y}{}}{dt}
    +
    N_{k_x,k_y}
    \kvel{k_x,k_y}{}
    =
    -
    C_{k_x,k_y}
    \kpos{k_x,k_y}{}
    +
    A_{k_x,k_y}.

To discretize it, we first multiply

.. math::

    \intfactor{t}
    \equiv
    \exp \left( N_{k_x,k_y} t \right)

with it (known as integrating factor) and integrate it in time to yield

.. math::

    \int_{t^n}^{t^{n+1}}
    \left\{
        \intfactor{t}
        \frac{dZ_{k_x,k_y}}{dt}
        +
        \intfactor{t}
        N_{k_x,k_y}
        \kvel{k_x,k_y}{}
    \right\}
    dt
    =
    -
    \int_{t^n}^{t^{n+1}}
    \intfactor{t}
    C_{k_x,k_y}
    \kpos{k_x,k_y}{}
    dt
    +
    \int_{t^n}^{t^{n+1}}
    \intfactor{t}
    A_{k_x,k_y}
    dt.

Due to

.. math::

    \intfactor{t}
    N_{k_x,k_y}
    =
    \frac{d}{dt}
    \left\{
        \intfactor{t}
    \right\},

the left-hand-side term leads to

.. math::

    \int_{t^n}^{t^{n+1}}
    \left[
        \intfactor{t}
        \frac{dZ_{k_x,k_y}}{dt}
        +
        \frac{d}{dt}
        \left\{
            \intfactor{t}
        \right\}
        \kvel{k_x,k_y}{}
    \right]
    dt
    =
    \left[
        \intfactor{t}
        Z_{k_x,k_y}
    \right]_{t^n}^{t^{n+1}},

and thus we obtain

.. math::

    \intfactor{t^{n + 1}}
    Z_{k_x,k_y}^{n + 1}
    -
    \intfactor{t^{n    }}
    Z_{k_x,k_y}^{n    }
    =
    -
    \int_{t^n}^{t^{n+1}}
    \intfactor{t}
    C_{k_x,k_y}
    \kpos{k_x,k_y}{}
    dt
    +
    \int_{t^n}^{t^{n+1}}
    \intfactor{t}
    A_{k_x,k_y}
    dt.

The first term in the right-hand side adopts the Crank-Nicolson scheme to achieve :ref:`the discrete energy conservation <appendix_discrete_energy_conservation>`:

.. math::

    -
    \int_{t^n}^{t^{n+1}}
    \intfactor{t}
    C_{k_x,k_y}
    \kpos{k_x,k_y}{}
    dt
    =
    -
    C_{k_x,k_y}
    \frac{1}{2}
    \left\{
        \intfactor{t^{n + 1}}
        \kpos{k_x,k_y}{n + 1}
        +
        \intfactor{t^{n    }}
        \kpos{k_x,k_y}{n    }
    \right\}
    \Delta t.

By dividing the relation by :math:`\intfactor{t^{n}}`, we obtain a conclusive scheme:

.. math::

    \intfactor{\Delta t}
    Z_{k_x,k_y}^{n + 1}
    -
    Z_{k_x,k_y}^{n    }
    =
    -
    C_{k_x,k_y}
    \frac{1}{2}
    \left\{
        \intfactor{\Delta t}
        \kpos{k_x,k_y}{n + 1}
        +
        \kpos{k_x,k_y}{n    }
    \right\}
    \Delta t
    +
    \int_{t^n}^{t^{n+1}}
    \intfactor{t - t^n}
    A_{k_x,k_y}
    dt.

The discretization of the external forcing term is arbitrary, and one possible way is elaborated in :ref:`elsewhere <appendix_external_forcing>`.

In summary, a set of discrete equations of interest is

.. math::

    \begin{pmatrix}
        1 & - \tau \\
        C_{k_x,k_y} \tau \intfactor{\Delta t} & \intfactor{\Delta t}
    \end{pmatrix}
    \begin{pmatrix}
        \kpos{k_x,k_y}{n + 1} \\
        \kvel{k_x,k_y}{n + 1}
    \end{pmatrix}
    =
    \begin{pmatrix}
        1 & \tau \\
        - C_{k_x,k_y} \tau & 1
    \end{pmatrix}
    \begin{pmatrix}
        \kpos{k_x,k_y}{n    } \\
        \kvel{k_x,k_y}{n    }
    \end{pmatrix}
    +
    \begin{pmatrix}
        0 \\
        \int_{t^n}^{t^{n+1}}
        \intfactor{t - t^n}
        A_{k_x,k_y}
        dt
    \end{pmatrix}.

