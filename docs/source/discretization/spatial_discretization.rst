We spatially discretize the equations in the spectral domain.
Instead of the superposition of infinite number of modes, we truncate it at the wavenumber of :math:`N - 1`; namely

.. math::

    \mysum{k}{0}{\infty}
    Q_k
    \approx
    \mysum{k}{0}{N - 1}
    Q_k.

Note that the transformation between physical and spectral representations leads to

.. math::

    q_n
    &
    =
    \frac{1}{N}
    \left\{
        \mysum{k}{0}{N - 2}
        Q_{k}
        \sin \left( \frac{\pi \left( n + \frac{1}{2} \right) \left( k + 1 \right)}{N} \right)
        +
        \left( - 1 \right)^{n}
        Q_{N - 1}
    \right\}
    \equiv
    \dstiii{Q_k},

    Q_k
    &
    =
    2
    \mysum{k}{0}{N - 1}
    q_n
    \sin \left( \frac{\pi \left( n + \frac{1}{2} \right) \left( k + 1 \right)}{N} \right)
    \equiv
    \dstii{q_n},

which are known as the discrete sine transform of type III and II, respectively.

