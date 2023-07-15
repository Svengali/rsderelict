
use anu;

use tracing::{span, Level, info, trace, debug, warn, error};

use tracing_subscriber::{fmt, prelude::*};

use tracing_subscriber::fmt::format::FmtSpan;
   

mod net;

mod layer;

//use anu::CustomLayer;

struct Tracer {
    
}

impl Tracer {
    pub fn span() {

    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        println!("Dropping Tracer!");
    }
    
}

fn main() 

{

    /*

    let fmt = tracing_subscriber::fmt()

    tracing_subscriber::registry().with(CustomLayer).init();

    .with_span_events(FmtSpan::FULL)

    .with_max_level(Level::DEBUG)

    .pretty()
    .compact()

    .init()
    ;
    */

    //tracing_subscriber::registry().with( anu::CustomLayer ).init();

    println!("Howdy.  Its a brand new world");
    println!("-----------------------------");

    tracing_subscriber::registry()
        .with( anu::CustomLayer { ..Default::default() } )
        //.with( anu::PrintVisitor )
        .init();

    /*

    let fmt = tracing_subscriber::fmt()

    .with_span_events(FmtSpan::FULL)

    .with_max_level(Level::DEBUG)

    .compact()

    .init()
    ;
    // */

    //println!("Registered Layer");

    info!("Testing each type of log level");
    trace!("Trace");
    debug!("Debug");
    info!("Info");
    warn!("Warn");
    error!("Error");

    info!(a_bool = true, answer = 42, message = "first example");

    net::Networking::test();

    {
        let main_span = span!(Level::INFO, "main()");
        let _span = main_span.enter();
    
        //println!("Hello, world!");
        info!( "Hello world" );

        anu::print( "Another span".to_string() );
    }



    anu::print( "Hello".to_string() );

    info!("End of application");

}
