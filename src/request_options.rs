// See https://lan.developer.lifx.com/docs/packet-contents for more details
pub struct LifxRequestOptions {
    /*
        The tagged field is a boolean flag that indicates whether the Frame Address target field is being used to address an
        individual device or all devices. When you broadcast a message to the network (i.e. broadcasting a GetService (2) 
        for discovery) you should set this field to 1 (true) and for all other messages you should set this to 0 (false).
    */
    pub tagged: bool,

    /*
        The source identifier allows each client to provide a unique value, which will be included in the corresponding field
        in Acknowledgement (45) and State packets the device sends back to you.

        Due to the behavior of some older versions of firmware you should set this value to anything other than zero or one.
        In some versions of the firmware a source value of 1 will be ignored. If you set source to 0 then the device may 
        broadcast the reply on port 56700 which can be received by all clients on the same subnet and may not be the port on
        which your client is listening for replies.
    */
    pub source: u32,

    /*
        The target field represents a device serial number and appears as a hex number of the form d073d5xxxxxx. When you
        address a device you should left-justify the first 6 bytes of the target field with the serial number and then 
        zero-fill the last two bytes. You should set this value to all zero's if you want to broadcast a message to the network.

        The replies you get back from devices will always contain the serial number of the device sending the reply. For 
        example, if you are discovering devices, the StateService (3) message will tell you the serial number for each LIFX 
        device on your network.
    */
    pub target: [u8; 8],


    /*
        It is recommended you set ack_required=1 and res_required=0 when you change the device with a Set message. 
        When you send a Get message it is best to set ack_required=0 and res_required=0, because these messages trigger an
        implicit State response. Note that when you ask for a response with a Set message that changes the visual state
        of the device, you will get the old values in the State message sent back. 
    */

    /*
        Setting this flag to true will cause the device to send an Acknowledgement (45) message
    */
    pub ack_required: bool,
    /*
        Setting this flag to true will make the device send back one or more State messages
    */
    pub res_required: bool,
    

    /*
        The sequence number allows the client to provide a unique value, which will be included by the LIFX device in any message
        that is sent in response to a message sent by the client. This allows the client to distinguish between different messages
        sent with the same source identifier in the Frame Header. We recommend that your program has one source value and keeps
        incrementing sequence per device for each message you send. Once sequence reaches the maximum value of 255 for that device,
        roll it back to 0 and keep incrementing from there.
    */
    pub sequence: u8,
}

impl LifxRequestOptions {
    pub fn default() -> LifxRequestOptions {
        LifxRequestOptions {
            tagged: false,
            source: 1234567890,
            target: [0; 8],
            res_required: false,
            ack_required: false,
            sequence: 0,
        }
    }

    pub fn increment_sequence(&mut self) {
        self.sequence = self.sequence.wrapping_add(1);
    }
}