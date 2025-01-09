//! This task handles DDS communications and relays them to the handler serial task.
use kingfisher_data_types::microcontroller_types::{MicroControlMessages, MicroStatusMessages};
use tokio::sync::mpsc;


// const CONTROL_READER_READY: Token = Token(0);
// const CONTROL_READER_STATUS_READY: Token = Token(1);
// const STATUS_WRITER_STATUS_READY: Token = Token(2);
// const SERIAL_STATUS_READY: Token = Token(3);

pub struct DDSTask {
    to_serial: mpsc::Sender<MicroControlMessages>,
    from_serial: mpsc::Receiver<MicroStatusMessages>
}

impl DDSTask {

    /// Create a new DDS Task
    pub fn new (to_serial: mpsc::Sender<MicroControlMessages>, from_serial: mpsc::Receiver<MicroStatusMessages>) -> Self {
        DDSTask {
            to_serial,
            from_serial
        }
    }

    /// Create a new DDS task.
    pub async fn run(&mut self) {


    }

    // Task that runs in separate thread and handles DDS communication.
    fn dds_loop(status_receiver: mpsc::Receiver<MicroStatusMessages>, control_to_serial: mpsc::Sender<MicroControlMessages>) {
        // let poll = Poll::new().unwrap();
        // let mut events = Events::with_capacity(4);

        // //create domain participant
        // let domain_participant = DomainParticipant::new(kingfisher_data_types::DEFAULT_DOMAIN)
        //     .unwrap_or_else(|e| panic!("DomainParticipant construction failed: {:?}", e));

        // //create quality of service
        // let mut qos = QosPolicyBuilder::new()
        //     .reliability( Reliability::Reliable {max_blocking_time: rustdds::Duration::DURATION_ZERO}).build();

        // //create control listener topic
        // let uc_control_topic = domain_participant
        //     .create_topic(
        //         kingfisher_data_types::dds_topics::MICROCONTROLLER_CONTROL_TOPIC.to_string(),
        //         "microcontroller_control".to_string(),
        //         &qos,
        //         TopicKind::NoKey
        //     ).unwrap_or_else(|e| panic!("create microcontroller control topic failed: {:?}", e));
        
        // let control_subscriber = domain_participant.create_subscriber(&qos).unwrap_or_else(|e| panic!("Creating the control subscriber failed: {:?}", e));
        // let mut control_reader = control_subscriber
        //   .create_datareader_no_key::<MicroControlMessages, CDRDeserializerAdapter<MicroControlMessages>>(&uc_control_topic, Some(qos))
        //   .unwrap_or_else(|e| panic!("Creating the control reader failed: {:?}", e));
        // //register reader with mio
        // poll.registry().register(&mut control_reader, CONTROL_READER_READY, Interest::READABLE)
        //   .unwrap_or_else(|e| panic!("Failed to register the reader ready event: {}", e));
        // poll.registry().register(
        //     control_reader.as_status_evented(),
        //     CONTROL_READER_STATUS_READY,
        //     Interest::READABLE
        //   ).unwrap_or_else(|e| panic!("Failed to register the control reader status ready event: {:?}", e));

        // //create status publisher
        // let uc_status_topic = domain_participant
        //     .create_topic(
        //         kingfisher_data_types::dds_topics::MICROCONTROLLER_STATUS_TOPIC.to_string(),
        //         "microcontroller_status".to_string(),
        //     &qos,
        //     TopicKind::NoKey,
        // ).unwrap_or_else(|e| panic!("Create uc status topic failed: {:?}", e));
        
        // let status_publisher = domain_participant.create_publisher(&qos).unwrap_or_else(|e| panic!("Create status publisher failed: {:?}", e));
        // let mut status_writer = status_publisher
        //     .create_datawriter_no_key::<MicroStatusMessages, CDRDeserializerAdapter<MicroStatusMessages>>(&uc_status_topic, None) // None = get qos policy from publisher
        //     .unwrap_or_else(|e| panic!("Create status writer failed: {:?}", e));
        // //register publisher with mio
        // poll.registry().register(
        //     status_writer.as_status_evented(),
        //     STATUS_WRITER_STATUS_READY,
        //     Interest::READABLE
        // ).unwrap_or_else(|e| panic!("Failed to register writer status even with mio: {:?}", e));

        // //register communication from serial port
        // poll.registry().register(
        //     &mut status_receiver,
        //     SERIAL_STATUS_READY,
        //     Interest::READABLE
        // ).unwrap_or_else(|e| panic!("Failed to register serial status channel with mio: {:?}", e));

        // let loop_delay = std::time::Duration::from_millis(10);
        // loop {
        //     poll.poll(&mut events, Some(loop_delay)).unwrap();
        //     for event in events {
        //         match event.token() {
        //             CONTROL_READER_READY => {
        //                 loop {
        //                     log::trace!("ControlReader triggered");
        //                     match control_reader.take_next_sample() {
        //                     Ok(Some(sample)) => match sample.into_value() {
        //                         Ok(sample) => {
        //                             log::info!("Received Control UC message: {:?}", sample);
        //                             control_to_serial.try_send(sample).unwrap_or_else(|e| log::error!("Failed to send control message to serial port: {:?}", e))
        //                         },
        //                         Err(key) => println!("Disposed key {:?}", key),
        //                     },
        //                     Ok(None) => break, // no more data
        //                     Err(e) => println!("DataReader error {:?}", e),
        //                     } 
        //                 }
        //             },
        //             CONTROL_READER_STATUS_READY => {
        //                 while let Some(status) = control_reader.try_recv_status() {
        //                     log::info!("ControlReader status: {:?}", status);
        //                 }
        //             },
        //             STATUS_WRITER_STATUS_READY => {
        //                 while let Some(status) = status_writer.try_recv_status() {
        //                     log::info!("StatusWriter status: {:?}", status);
        //                 }
        //             },
        //             SERIAL_STATUS_READY => {
        //                 match status_receiver.try_recv() {
        //                     Ok(status) => {
        //                         log::trace!("Status message received from serial and being sent over DDS.");
        //                         status_writer.write(status).unwrap_or_else(|e| log::error!("Failed to write uC status message to DDS bus: {:?}", e))
        //                     },
        //                     Err(e) => log::error!("Failed to get a status message from the serial port into the DDS thread: {:?}", e)
        //                 }
        //             },
        //             _ => ()
        //         }
        //     }
        // }
    }
}