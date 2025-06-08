/=  nock-verifier  /common/nock-verifier
/=  miner  /apps/dumbnet/miner
::
|%
+$  state  ~
+$  card   card:agent:gall
--
::
%-  agent:dbug
=|  state
=*  state  -
^-  agent:gall
|_  =bowl:gall
+*  this  .
    def   ~(. (default-agent this %.n) bowl)
::
++  on-init
  ^-  (quip card _this)
  ~&  >  %verifier-init
  `this
::
++  on-save   !>(state)
++  on-load   |=(old-state=vase `this)
++  on-poke
  |=  [=mark =vase]
  ^-  (quip card _this)
  ~&  >  "verifier poke: mark={<mark>}"
  ?+  mark  (on-poke:def mark vase)
    %noun
      =/  poke-data  !<(* vase)
      ~&  >  "verifier poke data: {<poke-data>}"
      ?+  poke-data  `this
        ::  Handle verification request
        [%verify *]
          =/  [=proof override=(unit (list term)) eny=@]  
            ?>  ?=([%verify *] poke-data)
            +.poke-data
          ~&  >  "verifying proof with entropy: {<eny>}"
          =/  result=?  (verify:nock-verifier proof override eny)
          ~&  >  "verification result: {<result>}"
          `this
        ::  Handle mining request (delegate to miner)
        [%mine *]
          ~&  >  "delegating to miner"
          =/  mine-data  +.poke-data
          (on-poke:miner mark !>(mine-data))
      ==
  ==
::
++  on-watch  on-watch:def
++  on-leave  on-leave:def
++  on-peek   on-peek:def
++  on-agent  on-agent:def
++  on-arvo   on-arvo:def
++  on-fail   on-fail:def
--
