########
Appendix
########

.. _appendix_discrete_energy_conservation:

****************************
Discrete energy conservation
****************************

In the absence of additional factors (dissipation, external forcing), the total energy should be conserved.
The discrete equations discussed in the previous section are designed to satisfy this property, which is confirmed here.

The simplified discrete equations are

.. math::

    \begin{pmatrix}
        1 & - \tau \\
        C_{k_x,k_y} \tau & 1
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
    \end{pmatrix}.

We define the discrete total kinetic and potential energies as

.. math::

    K
    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    \left( \kvel{k_x,k_y}{} \right)^2,

    U
    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    C_{k_x,k_y}
    \left( \kpos{k_x,k_y}{} \right)^2,

which are identical to the theoretical relations but truncated at :math:`k_x = N_x` and :math:`k_y = N_y`.

Their differentiations (in time) yield

.. math::

    \dif{K}{t}
    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    \frac{
        \left( \kvel{k_x,k_y}{n + 1} \right)^2
        -
        \left( \kvel{k_x,k_y}{n    } \right)^2
    }{\Delta t}

    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \rho
    \frac{
        \kvel{k_x,k_y}{n + 1}
        -
        \kvel{k_x,k_y}{n    }
    }{\Delta t}
    \frac{
        \kvel{k_x,k_y}{n + 1}
        +
        \kvel{k_x,k_y}{n    }
    }{2}

    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \rho
    \dif{\kvel{k_x,k_y}{}}{t}
    \ave{\kvel{k_x,k_y}{}},

    \dif{U}{t}
    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    C_{k_x,k_y}
    \frac{
        \left( \kpos{k_x,k_y}{n + 1} \right)^2
        -
        \left( \kpos{k_x,k_y}{n    } \right)^2
    }{\Delta t}

    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \rho
    C_{k_x,k_y}
    \dif{\kpos{k_x,k_y}{}}{t}
    \ave{\kpos{k_x,k_y}{}}

    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \rho
    C_{k_x,k_y}
    \ave{\kvel{k_x,k_y}{}}
    \,
    \ave{\kpos{k_x,k_y}{}},

and, as a consequence:

.. math::

    \dif{E}{t}
    &
    =
    \dif{K}{t}
    +
    \dif{U}{t}

    &
    =
    \mysum{k_y}{N_y}
    \mysum{k_x}{N_x}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \rho
    \ave{\kvel{k_x,k_y}{}}
    \left(
        \dif{\kvel{k_x,k_y}{}}{t}
        +
        C_{k_x,k_y}
        \ave{\kpos{k_x,k_y}{}}
    \right).

Since we design the discrete equation of motion such that the term inside the parentheses is zero, we see that our scheme satisfies the discrete energy conservation:

.. math::

    \dif{E}{t}
    =
    0.

.. _appendix_external_forcing:

****************
External forcing
****************

The simplest approach to discretize the term is to adopt the Euler-forward scheme, yielding

.. math::

    \int_{t^n}^{t^{n+1}}
    \intfactor{t - t^n}
    A_{k_x,k_y}
    dt
    \approx
    A_{k_x,k_y}^{n}
    \Delta t.

Although external forcing can be arbitrarily prescribed, we limit our focus to a `sound source`, i.e., a specific profile in physical space is given, whose amplitude only changes in time:

.. math::

    \int_{t^n}^{t^{n+1}}
    \intfactor{t - t^n}
    A_{k_x,k_y}
    dt
    =
    A_{k_x,k_y}
    \int_{t^n}^{t^{n+1}}
    \intfactor{t - t^n}
    \sin \left( \omega t \right)
    dt.

Due to

.. math::

    &
    \left\{
        1
        +
        \left(
            \frac{N_{k_x,k_y}}{\omega}
        \right)^2
    \right\}
    \int_{t^n}^{t^{n+1}}
    \intfactor{t - t^n}
    \sin \left( \omega t \right)
    dt

    =
    &
    \left[
        -
        \frac{1}{\omega}
        \intfactor{t - t^n}
        \cos \left( \omega t \right)
        +
        \frac{N_{k_x,k_y}}{\omega^2}
        \intfactor{t - t^n}
        \sin \left( \omega t \right)
    \right]_{t^n}^{t^{n + 1}}

    =
    &
    \intfactor{\Delta t}
    \left\{
        -
        \frac{1}{\omega}
        \cos \left( \omega t^{n + 1} \right)
        +
        \frac{N_{k_x,k_y}}{\omega^2}
        \sin \left( \omega t^{n + 1} \right)
    \right\}
    -
    \left\{
        -
        \frac{1}{\omega}
        \cos \left( \omega t^{n    } \right)
        +
        \frac{N_{k_x,k_y}}{\omega^2}
        \sin \left( \omega t^{n    } \right)
    \right\},

we are able to evaluate it analytically.

