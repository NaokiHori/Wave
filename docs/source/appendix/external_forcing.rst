The simplest approach to discretize the term is to adopt the Euler-forward scheme, yielding

.. math::

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
    \approx
    A_k^{n}
    \Delta t.

Although external forcing can be arbitrarily prescribed, we limit our focus to a `sound source`, i.e., a specific profile in physical space is given, whose amplitude only changes in time:

.. math::

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
    =
    A_k
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t - t^n}
        \sin \left( \omega t \right)
    }{
        t
    }.

Due to

.. math::

    &
    \left(
        1
        +
        \frac{N_k^2}{\omega^2}
    \right)
    \myint{
        t^n
    }{
        t^{n+1}
    }{
        \intfactor{t - t^n}
        \sin \left( \omega t \right)
    }{
        t
    }

    =
    &
    \left[
        -
        \frac{1}{\omega}
        \intfactor{t - t^n}
        \cos \left( \omega t \right)
        +
        \frac{N_k}{\omega^2}
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
        \frac{N_k}{\omega^2}
        \sin \left( \omega t^{n + 1} \right)
    \right\}
    -
    \left\{
        -
        \frac{1}{\omega}
        \cos \left( \omega t^{n    } \right)
        +
        \frac{N_k}{\omega^2}
        \sin \left( \omega t^{n    } \right)
    \right\},

we are able to evaluate it analytically.

