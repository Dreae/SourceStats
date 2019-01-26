@0xd5461e70f0ced87f;

struct PlayerUpdate {
    steamId64 @0 :Int64;
    shots @1 :List(ShotFired);
    kills @2 :List(Kill);

    struct ShotFired {
        map @0 :Text;
        posX @1 :Int32;
        posY @2 :Int32;
        posZ @3 :Int32;
        hit @4 :Bool;
        weapon @5 :Int16;
        headshot @6 :Bool;
        timestamp @7 :Int64;
    }

    struct Kill {
        map @0 :Text;
        posX @1 :Int32;
        posY @2 :Int32;
        posZ @3 :Int32;
        other @4 :Int64;
        headshot @5 :Bool;
        weapon @6 :Int16;
        timestamp @7 :Int64;
    }
}