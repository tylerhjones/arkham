# Example Project Goals
````
let runtime = 
  Arkham::new()
    .provider( provider here )
    .provider( provider here )
    
    .addRestResource()
      .context("/foo")
      .connector("")
      .resource(ObjectKey(FooResource.id))
      .filter(FilterObject.id)
      .filter(AnotherFilterObject.id)
      .exceptionMapper(Mapper.id)
      .build()
      
    .addRestResource()
      .context("/bar")
      .connector(
          Connector(
              address supplier, 
              port))
      .resource(
          ObjectKey(
             BarResource.id))
      .filter(FilterObject.id)
      .filter(AnotherFilterObject.id)
      .exceptionMapper(Mapper.id)
      .build()
      
      // other goals
    .addLifecycled(...?)
    
    .build();

runtime.start();
````

Main point being to do all setup / assembly using the providers similar to guice, 
and to view the entire app setup from one page
