actor {
    public shared(msg) func whoami() : async Principal {
        return msg.caller;
    };
};
