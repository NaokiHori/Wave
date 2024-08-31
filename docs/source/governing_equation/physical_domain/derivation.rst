We consider a one-dimensional system where masses are connected with springs and dampers.
For simplicity, we assume that the mass :math:`\dimvar{m}`, the spring constant :math:`\dimvar{k}`, and the damping coefficient :math:`\dimvar{\mu}` take identical values for now.

Due to the force balance for each mass, we obtain a set of equations which govern the dynamics as

.. math::

    \tder{
        \dimvar{\pos{}{}}
    }{
        \dimvar{t}
    }
    =
    &
    \dimvar{\vel{}{}},

    \dimvar{m}
    \tder{
        \dimvar{\vel{}{}}
    }{
        \dimvar{t}
    }
    =
    &
    \dimvar{k}
    \left\{
        \dimvar{\pos{}{}} \left( \dimvar{x} - \Delta \dimvar{x} \right)
        -
        \dimvar{\pos{}{}} \left( \dimvar{x}                     \right)
    \right\}
    +
    \dimvar{k}
    \left\{
        \dimvar{\pos{}{}} \left( \dimvar{x} + \Delta \dimvar{x} \right)
        -
        \dimvar{\pos{}{}} \left( \dimvar{x}                     \right)
    \right\}

    +
    &
    \dimvar{\mu}
    \left\{
        \dimvar{\vel{}{}} \left( \dimvar{x} - \Delta \dimvar{x} \right)
        -
        \dimvar{\vel{}{}} \left( \dimvar{x}                     \right)
    \right\}
    +
    \dimvar{\mu}
    \left\{
        \dimvar{\vel{}{}} \left( \dimvar{x} + \Delta \dimvar{x} \right)
        -
        \dimvar{\vel{}{}} \left( \dimvar{x}                     \right)
    \right\}

    +
    &
    \dimvar{m} \dimvar{a},

where :math:`\dimvar{\pos{}{}}` and :math:`\dimvar{\vel{}{}}` are the displacements and velocities, respectively, and :math:`\dimvar{m} \dimvar{a}` is the external forcing.

In the limit of :math:`\Delta \dimvar{x} \rightarrow 0`, by utilizing the Taylor series expansion, the second relation converges to

.. math::

    \pder{
        \dimvar{\vel{}{}}
    }{
        \dimvar{t}
    }
    =
    \dimvar{c}^2
    \pder{}{\dimvar{x}}
    \pder{
        \dimvar{\pos{}{}}
    }{
        \dimvar{x}
    }
    +
    \dimvar{\nu}
    \pder{}{\dimvar{x}}
    \pder{
        \dimvar{\vel{}{}}
    }{
        \dimvar{x}
    }
    +
    \dimvar{a},

where

.. math::

    \dimvar{c}
    &
    \equiv
    \sqrt{
        \frac{
            \dimvar{k} \Delta \dimvar{x}^2
        }{
            \dimvar{m}
        }
    },

    \dimvar{\nu}
    &
    \equiv
    \frac{
        \dimvar{\mu} \Delta \dimvar{x}^2
    }{
        \dimvar{m}
    }

are the propagation speed and the kinematic viscosity having the dimensions of :math:`\left[ L T^{-1} \right]` and :math:`\left[ L^2 T^{-1} \right]`, respectively, which are to be given as material properties.

By using a reference length scale :math:`\dimvar{L}` and a time scale :math:`\dimvar{T}`, we obtain a set of non-dimensional governing equations:

.. math::

    \pder{\pos{}{}}{t}
    &
    =
    \vel{}{},

    \pder{\vel{}{}}{t}
    &
    =
    c^2
    \pder{}{x}
    \pder{\pos{}{}}{x}
    +
    \nu
    \pder{}{x}
    \pder{\vel{}{}}{x}
    +
    a,

where the following non-dimensional numbers are involved:

.. math::

    t
    &
    \equiv
    \frac{
        1
    }{
        \dimvar{T}
    }
    \dimvar{t},

    \pos{}{}
    &
    \equiv
    \frac{
        1
    }{
        \dimvar{L}
    }
    \dimvar{\pos{}{}},

    \vel{}{}
    &
    \equiv
    \frac{
        \dimvar{T}
    }{
        \dimvar{L}
    }
    \dimvar{\vel{}{}},

    c
    &
    \equiv
    \frac{
        \dimvar{T}
    }{
        \dimvar{L}
    }
    \dimvar{c},

    \nu
    &
    \equiv
    \frac{
        \dimvar{T}
    }{
        \dimvar{L}^2
    }
    \dimvar{\nu},

    a
    &
    \equiv
    \frac{
        \dimvar{T}^2
    }{
        \dimvar{L}
    }
    \dimvar{a}.

For multi-dimensional domains, we have a straightforward extension:

.. math::

    \pder{\pos{}{}}{t}
    &
    =
    \vel{}{},

    \pder{\vel{}{}}{t}
    &
    =
    c^2
    \pder{}{x_i}
    \pder{\pos{}{}}{x_i}
    +
    \nu
    \pder{}{x_i}
    \pder{\vel{}{}}{x_i}
    +
    a,

where the summation rule is adopted for brevity.

