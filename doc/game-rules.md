# global view

This is a poc, it must be as simple as possible but also
show as many use cases as possible.

## uses cases demonstrate in this poc : 

- [x] use [hfb-auth](https://github.com/horfimbor/hfb-auth) to authenticate users
- [x] do not use cookies and allow to use one account on each browsers tabs
- [ ] use refresh token to stay connected
- [ ] use rooting to go back to the pages
- [X] use multiple ms (micro-services)  linked only by public event and web components
- [ ] allow multiple ms A depends on the same ms B and then depend on different ms C
- [ ] game server time using [horfimbor-time](https://github.com/horfimbor/horfimbor-engine/tree/main/horfimbor-time)
- [ ] use delayed events that can be cancelled by the user
- [ ] use bots

## description of microservices :

### civilisation

this ms is responsible for:
- the login
- the definition of time
- the name and description of the civilisation
- the list of planets
- the kind of army

### planet

this ms is responsible for:
- the population
- the buildings
- the production of:
  - population
  - building
  - battleships
- the list of armies
- the history of fights
- the current fight

### army easy

- the list of available ships (only 3 kinds) and their cost
- the movement of the army
- the number of lost before withdrawing
- fight are a simple shi fu mi one shot

### army hard

- the list of available ships and their cost
- the movement of the army
- the list of available ships
- each ship shot every X seconds, one event is generated at those times.

