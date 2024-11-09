We spatially discretize the equations in the spectral domain.
Instead of the superposition of infinite number of modes, we pick up :math:`N` modes:

.. math::

    \mysum{k}{-\infty}{\infty}
    Q_k
    \approx
    \mysum{k}{-N / 2}{N / 2 - 1}
    Q_k.

Note that the transformation between physical and spectral representations leads to

.. math::

    q_n
    &
    =
    \frac{1}{N}
    \mysum{k}{0}{N - 1}
    Q_k
    \exp \left( \wavenumber{2 \pi}{n k}{N} I \right)
    \equiv
    \dftiii{Q_k},

    Q_k
    &
    =
    \mysum{n}{0}{N - 1}
    q_n
    \exp \left( \wavenumber{- 2 \pi}{n k}{N} I \right)
    \equiv
    \dftii{q_n},

which are known as the discrete sine transform of type III and II, respectively.

