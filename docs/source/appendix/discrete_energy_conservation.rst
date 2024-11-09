In the absence of additional factors (dissipation, external forcing), the total energy should be conserved.
The discrete equations discussed in :ref:`the temporal discretization <temporal_discretization>` are designed to satisfy this property, which is confirmed here.

The simplified discrete equations are

.. math::

    \begin{pmatrix}
        1 & - \tau \\
        C_k \tau & 1
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
    \end{pmatrix}.

We define the discrete total kinetic and potential energies as

.. math::

    K
    &
    =
    \mysum{k}{0}{N - 1}
    L
    \frac{1}{2}
    \rho
    \left( \kvel{k}{} \right)^2,

    U
    &
    =
    \mysum{k}{0}{N - 1}
    L
    \frac{1}{2}
    \rho
    C_k
    \left( \kpos{k}{} \right)^2,

which are identical to the theoretical relations but truncated at :math:`k = N`.

Their differentiations (in time) yield

.. math::

    \dif{K}{t}
    &
    =
    \mysum{k}{0}{N - 1}
    L
    \frac{1}{2}
    \rho
    \frac{
        \left( \kvel{k}{n + 1} \right)^2
        -
        \left( \kvel{k}{n    } \right)^2
    }{\Delta t}

    &
    =
    \mysum{k}{0}{N - 1}
    L
    \rho
    \frac{
        \kvel{k}{n + 1}
        -
        \kvel{k}{n    }
    }{\Delta t}
    \frac{
        \kvel{k}{n + 1}
        +
        \kvel{k}{n    }
    }{2}

    &
    =
    \mysum{k}{0}{N - 1}
    L
    \rho
    \dif{\kvel{k}{}}{t}
    \ave{\kvel{k}{}},

    \dif{U}{t}
    &
    =
    \mysum{k}{0}{N - 1}
    L
    \frac{1}{2}
    \rho
    C_k
    \frac{
        \left( \kpos{k}{n + 1} \right)^2
        -
        \left( \kpos{k}{n    } \right)^2
    }{\Delta t}

    &
    =
    \mysum{k}{0}{N - 1}
    L
    \rho
    C_k
    \dif{\kpos{k}{}}{t}
    \ave{\kpos{k}{}}

    &
    =
    \mysum{k}{0}{N - 1}
    L
    \rho
    C_k
    \ave{\kvel{k}{}}
    \,
    \ave{\kpos{k}{}},

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
    \mysum{k}{0}{N - 1}
    L
    \rho
    \ave{\kvel{k}{}}
    \left(
        \dif{\kvel{k}{}}{t}
        +
        C_k
        \ave{\kpos{k}{}}
    \right).

Since we design the discrete equation of motion such that the term inside the parentheses is zero, we see that our scheme satisfies the discrete energy conservation:

.. math::

    \dif{E}{t}
    =
    0.

