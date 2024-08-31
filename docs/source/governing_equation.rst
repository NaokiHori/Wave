##################
Governing equation
##################

*********************
One-dimensional cases
*********************

==============================
Derivation and energy relation
==============================

We consider a system where masses are connected with springs and dampers.
For simplicity, we assume that the mass :math:`m`, the spring constant :math:`k`, and the damping coefficient :math:`\eta` take identical values for now.

Due to the force balance for each mass, we obtain a set of equations which govern the dynamics as

.. math::

    \frac{d\pos{}{}}{dt}
    &
    =
    \vel{}{},

    m
    \frac{d\vel{}{}}{dt}
    &
    =
    \force{}

    &
    =
    k
    \left\{
        \pos{}{} \left( x - \Delta x \right)
        -
        \pos{}{} \left( x            \right)
    \right\}
    +
    k
    \left\{
        \pos{}{} \left( x + \Delta x \right)
        -
        \pos{}{} \left( x            \right)
    \right\}

    &
    +
    \eta
    \left\{
        \vel{}{} \left( x - \Delta x \right)
        -
        \vel{}{} \left( x            \right)
    \right\}
    +
    \eta
    \left\{
        \vel{}{} \left( x + \Delta x \right)
        -
        \vel{}{} \left( x            \right)
    \right\}

    &
    +
    m a,

where :math:`\pos{}{}` and :math:`\vel{}{}` are the displacements and velocities, respectively, and :math:`m a` is the external forcing.

In the limit of :math:`\Delta x \rightarrow 0`, by utilizing the Taylor series expansion, the second relation converges to

.. math::

    \frac{\partial \vel{}{}}{\partial t}
    =
    \frac{\partial^2 \pos{}{}}{\partial t^2}
    &
    =
    \frac{k \Delta x^2}{m}
    \frac{\partial^2 \pos{}{}}{\partial x^2}
    +
    \frac{\eta \Delta x^2}{m}
    \frac{\partial^2 \vel{}{}}{\partial x^2}
    +
    a

    &
    =
    c^2
    \frac{\partial^2 \pos{}{}}{\partial x^2}
    +
    \nu
    \frac{\partial^2 \vel{}{}}{\partial x^2}
    +
    a,

where :math:`c` and :math:`\nu` are coefficients having the dimensions of :math:`\left[ L T^{-1} \right]` and :math:`\left[ L^2 T^{-1} \right]`, respectively.

At the boundaries, we impose Dirichlet boundary conditions:

.. _boundary_condition:

.. math::

    &
    \pos{}{}
    =
    0,

    &
    \vel{}{}
    =
    0,

to describe the boundaries being fixed.

To see the energy balance, we multiply :math:`\rho \vel{}{}` with the equation of motion and volume-integrate it:

.. math::

    \int_{0}^{L_x}
    \rho
    \vel{}{}
    \frac{\partial \vel{}{}}{\partial t}
    dx
    -
    \int_{0}^{L_x}
    \rho
    \vel{}{}
    c^2
    \frac{\partial^2 \pos{}{}}{\partial x^2}
    dx
    -
    \int_{0}^{L_x}
    \rho
    \vel{}{}
    \nu
    \frac{\partial^2 \vel{}{}}{\partial x^2}
    dx
    -
    \int_{0}^{L_x}
    \rho
    \vel{}{}
    a
    dx
    =
    0,

where :math:`L_x` is the length of the domain.

With some algebra, we see that the first term is equal to

.. math::

    \frac{\partial}{\partial t}
    \left(
        \int_{0}^{L_x}
        \frac{1}{2}
        \rho
        \vel{}{}^2
        dx
    \right)
    =
    \frac{dK}{dt},

which describes the change of the total kinetic energy :math:`K` in time.

To reformulate the second term, we adopt the integration by parts:

.. math::

    -
    \int_{0}^{L_x}
    \rho
    c^2
    \vel{}{}
    \frac{\partial^2 \pos{}{}}{\partial x^2}
    dx
    =
    -
    \left[
        \rho
        c^2
        \vel{}{}
        \frac{\partial \pos{}{}}{\partial x}
    \right]_{0}^{L_x}
    +
    \int_{0}^{L_x}
    \rho
    c^2
    \frac{\partial \vel{}{}}{\partial x}
    \frac{\partial \pos{}{}}{\partial x}
    dx,

where the former term denotes the boundary contribution and is zero due to :ref:`the prescribed condition <boundary_condition>`, while the latter term can be written as

.. math::

    \int_{0}^{L_x}
    \rho
    c^2
    \frac{\partial}{\partial t}
    \left(
        \frac{\partial \pos{}{}}{\partial x}
    \right)
    \frac{\partial \pos{}{}}{\partial x}
    dx
    =
    \frac{\partial}{\partial t}
    \left(
        \int_{0}^{L_x}
        \frac{1}{2}
        \rho
        c^2
        \frac{\partial \pos{}{}}{\partial x}
        \frac{\partial \pos{}{}}{\partial x}
        dx
    \right)
    =
    \frac{dU}{dt},

which describes the change of the total potential energy :math:`U` in time.

Similarly, the third term can be reformulated by adopting the integration by parts:

.. math::

    -
    \int_{0}^{L_x}
    \rho
    \nu
    \vel{}{}
    \frac{\partial^2 \vel{}{}}{\partial x^2}
    dx
    =
    -
    \left[
        \rho
        \nu
        \vel{}{}
        \frac{\partial \vel{}{}}{\partial x}
    \right]_{0}^{L_x}
    +
    \int_{0}^{L_x}
    \rho
    \nu
    \frac{\partial \vel{}{}}{\partial x}
    \frac{\partial \vel{}{}}{\partial x}
    dx,

where the former term is again zero whereas the latter denotes the energy dissipation.

The fourth term is simply the energy input due to the external factor and remain as it is.

In summary, we obtain

.. math::

    \frac{dE}{dt}
    =
    \frac{d}{dt}
    \left(
        K
        +
        U
    \right)
    =
    -
    \int_{0}^{L_x}
    \rho
    \nu
    \frac{\partial \vel{}{}}{\partial x}
    \frac{\partial \vel{}{}}{\partial x}
    dx
    +
    \int_{0}^{L_x}
    \rho
    \vel{}{}
    a
    dx.

=======================
Spectral representation
=======================

The use of :ref:`Dirichlet boundary conditions <boundary_condition>` motivates the expansion of the fields using sinusoidal functions:

.. math::

    \pos{}{} \left( x, t \right)
    =
    \mysum{l_x}{\infty}
    \kpos{l_x}{} \left( t \right)
    \sin \left( \wavenumber{l_x}{x}{L_x} \right),

.. math::

    \vel{}{} \left( x, t \right)
    =
    \mysum{l_x}{\infty}
    \kvel{l_x}{} \left( t \right)
    \sin \left( \wavenumber{l_x}{x}{L_x} \right).

Utilizing these relations to the governing equations yields

.. math::

    &
    \mysum{l_x}{\infty}
    \frac{d\kpos{l_x}{}}{dt}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right)
    =
    \mysum{l_x}{\infty}
    \kvel{l_x}{}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right),

    &
    \mysum{l_x}{\infty}
    \frac{d\kvel{l_x}{}}{dt}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right)
    =
    -
    \mysum{l_x}{\infty}
    c^2
    \left( \wavenumber{l_x}{}{L_x} \right)^2
    \kpos{l_x}{}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right)
    -
    \mysum{l_x}{\infty}
    \nu
    \left( \wavenumber{l_x}{}{L_x} \right)^2
    \kvel{l_x}{}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right)
    +
    \mysum{l_x}{\infty}
    A_{l_x}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right),

and by adopting the orthogonality of the trial function:

.. math::

    &
    \int_{0}^{L_x}
    q \left( x \right)
    \sin \left( \wavenumber{k_x}{x}{L_x} \right)
    dx

    =
    &
    \int_{0}^{L_x}
    \mysum{l_x}{\infty}
    Q_{l_x}
    \sin \left( \wavenumber{l_x}{x}{L_x} \right)
    \sin \left( \wavenumber{k_x}{x}{L_x} \right)
    dx

    =
    &
    \int_{0}^{L_x}
    \mysum{l_x}{\infty}
    Q_{l_x}
    \frac{1}{2}
    \cos \left( \wavenumber{\left( l_x - k_x \right)}{x}{L_x} \right)
    dx
    -
    \int_{0}^{L_x}
    \mysum{l_x}{\infty}
    Q_{l_x}
    \frac{1}{2}
    \cos \left( \wavenumber{\left( l_x + k_x \right)}{x}{L_x} \right)
    dx

    =
    &
    \frac{L_x}{2}
    Q_{k_x},

we see that each wavenumber has independent relations:

.. math::

    &
    \frac{d\kpos{k_x}{}}{dt}
    =
    \kvel{k_x}{}

    &
    \frac{d\kvel{k_x}{}}{dt}
    =
    -
    C_{k_x}
    \kpos{k_x}{}
    -
    N_{k_x}
    \kvel{k_x}{}
    +
    A_{k_x},

where

.. math::

    C_{k_x}
    &
    \equiv
    c^2
    \left( \wavenumber{k_x}{}{L_x} \right)^2,

    N_{k_x}
    &
    \equiv
    \nu
    \left( \wavenumber{k_x}{}{L_x} \right)^2.

The total kinetic and potential energies (:math:`K` and :math:`U`) in spectral space lead to

.. math::

    K
    &
    \equiv
    \int_{0}^{L_x}
    \frac{1}{2}
    \rho
    \vel{}{}^2
    dx

    &
    =
    \int_{0}^{L_x}
    \frac{1}{2}
    \rho
    \left(
        \mysum{k_x}{\infty}
        \kvel{k_x}{}
        \sin \left( \wavenumber{k_x}{x}{L_x} \right)
    \right)
    \left(
        \mysum{l_x}{\infty}
        \kvel{l_x}{}
        \sin \left( \wavenumber{l_x}{x}{L_x} \right)
    \right)
    dx

    &
    =
    \mysum{k_x}{\infty}
    \frac{L_x}{2}
    \frac{1}{2}
    \rho
    \left( \kvel{k_x}{} \right)^2,

    U
    &
    \equiv
    \int_{0}^{L_x}
    \frac{1}{2}
    \rho
    c^2
    \frac{\partial \pos{}{}}{\partial x}
    \frac{\partial \pos{}{}}{\partial x}
    dx

    &
    =
    \int_{0}^{L_x}
    \frac{1}{2}
    \rho
    c^2
    \left(
        \mysum{k_x}{\infty}
        \kpos{k_x}{}
        \wavenumber{k_x}{}{L_x}
        \cos \left( \wavenumber{k_x}{x}{L_x} \right)
    \right)
    \left(
        \mysum{l_x}{\infty}
        \kpos{l_x}{}
        \wavenumber{l_x}{}{L_x}
        \cos \left( \wavenumber{l_x}{x}{L_x} \right)
    \right)
    dx

    &
    =
    \mysum{k_x}{\infty}
    \frac{L_x}{2}
    \frac{1}{2}
    \rho
    C_{k_x}
    \left( \kpos{k_x}{} \right)^2,

respectively.

***********************
Multi-dimensional cases
***********************

A multi-dimensional extension of the above discussion is straightforward; namely the velocity is given by the temporal derivative of the displacement, the force balance leads to

.. math::

    \frac{\partial \vel{}{}}{\partial t}
    =
    \frac{\partial^2 \pos{}{}}{\partial t^2}
    =
    c^2
    \frac{\partial}{\partial x_i}
    \frac{\partial \pos{}{}}{\partial x_i}
    +
    \nu
    \frac{\partial}{\partial x_i}
    \frac{\partial \vel{}{}}{\partial x_i}
    +
    a,

and the energy balance is given by

.. math::

    \frac{\partial}{\partial t}
    \left(
        \int
        \frac{1}{2}
        \rho
        \vel{}{}^2
        dV
    \right)
    +
    \frac{\partial}{\partial t}
    \left(
        \int
        \frac{1}{2}
        \rho
        c^2
        \frac{\partial \pos{}{}}{\partial x_i}
        \frac{\partial \pos{}{}}{\partial x_i}
        dV
    \right)
    =
    -
    \int
    \rho
    \nu
    \frac{\partial \vel{}{}}{\partial x_i}
    \frac{\partial \vel{}{}}{\partial x_i}
    dV
    +
    \int
    \rho
    \vel{}{}
    a
    dV,

where

.. math::

    \int
    q
    dV
    \equiv
    \int_{0}^{L_y}
    \int_{0}^{L_x}
    q
    dx
    dy

for two-dimensional cases, and same applies to higher dimensions.

In the spectral space, the displacement and velocity fields are expanded as

.. math::

    &
    \pos{}{} \left( x, y, t \right)
    =
    \mysum{k_y}{\infty}
    \mysum{k_x}{\infty}
    \kpos{k_x,k_y}{} \left( t \right)
    \sin \left( \wavenumber{k_x}{x}{L_x} \right)
    \sin \left( \wavenumber{k_y}{y}{L_y} \right),

    &
    \vel{}{} \left( x, y, t \right)
    =
    \mysum{k_y}{\infty}
    \mysum{k_x}{\infty}
    \kvel{k_x,k_y}{} \left( t \right)
    \sin \left( \wavenumber{k_x}{x}{L_x} \right)
    \sin \left( \wavenumber{k_y}{y}{L_y} \right).

The governing equations in spectral space lead to

.. math::

    \frac{d\kpos{k_x,k_y}{}}{dt}
    =
    &
    \kvel{k_x,k_y}{},

    \frac{d\kvel{k_x,k_y}{}}{dt}
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
    \left\{
        \left( \wavenumber{k_x}{}{L_x} \right)^2
        +
        \left( \wavenumber{k_y}{}{L_y} \right)^2
    \right\},

    N_{k_x,k_y}
    &
    \equiv
    \nu
    \left\{
        \left( \wavenumber{k_x}{}{L_x} \right)^2
        +
        \left( \wavenumber{k_y}{}{L_y} \right)^2
    \right\}.

The total energies are

.. math::

    K
    &
    =
    \mysum{k_y}{\infty}
    \mysum{k_x}{\infty}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    \left( \kvel{k_x,k_y}{} \right)^2,

    U
    &
    =
    \mysum{k_y}{\infty}
    \mysum{k_x}{\infty}
    \frac{L_x}{2}
    \frac{L_y}{2}
    \frac{1}{2}
    \rho
    C_{k_x,k_y}
    \left( \kpos{k_x,k_y}{} \right)^2.

