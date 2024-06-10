use keyberon::action::{k, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

static EXTRALAYER: Action = Action::DefaultLayer(3);
static QWERTZLAYER: Action = Action::DefaultLayer(0);

const TIMEOUT: u16 = 200;


const CTRL_SP: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Space),
});

const ALT_ENT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(Enter),
});

const PPN: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(MediaNextSong),
    tap: k(MediaPlayPause),
});


macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k].as_slice())
    };
}
macro_rules! a {
    ($k:ident) => {
        m(&&[RAlt, $k].as_slice())
    };
}

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    {
        [ Tab    Q   W   E   R   T     Y   U   I   O   P      LBracket]
        [ Escape A   S   D   F   G     H   J   K   L   SColon Quote   ]
        [ LShift Z   X   C   V   B     N   M   ,   .   Slash  RShift  ]
        [ n n n LCtrl {ALT_ENT} (2)   (1) {CTRL_SP} LAlt n n  n           ]
    }{
        [ t {a!(E)}     {s!(Grave)} {a!(Kb8)}         {a!(Kb9)}      Grave          {s!(Kb1)}   NonUsBslash {s!(NonUsBslash)} {s!(Kb0)}       {s!(Kb6)}   BSpace  ],
        [ t {a!(Minus)} {s!(Kb7)}   {a!(Kb7)}         {a!(Kb0)}      {s!(RBracket)} {s!(Minus)} {s!(Kb8)}   {s!(Kb9)}         Slash           {s!(Dot)}   {a!(Q)} ],
        [ t NonUsHash   {s!(Kb4)}   {a!(NonUsBslash)} {a!(RBracket)} {s!(Equal)}    RBracket    {s!(Kb5)}   {s!(Kb2)}         {s!(NonUsHash)} {s!(Comma)} t       ],
        [ t t           t           t                 t              t              {EXTRALAYER} t          t                 t               t           t       ],
    }{
        [ t  PgUp   Delete  Up  BSpace PgDown         -        Kb7 Kb8 Kb9 Minus BSpace ],
        [ t  Home   Left   Down Right  End            Kb0      Kb4 Kb5 Kb6 Comma Dot    ],
        [ t  LGui   Tab    Mute Enter  LGui           RBracket Kb1 Kb2 Kb3 LGui  t      ],
        [ t  t      t      t    t      {EXTRALAYER}   t        t   t   t   t     t      ],
    }{
        [{Custom(())}  n    n     {PPN} VolUp    n             PScreen    F7  F8  F9  F10  {Custom(())}],
        [{QWERTZLAYER} n    n     n     Mute     n            ScrollLock F4  F5  F6   F11  t           ],
        [n             A    n     n     VolDown  n            Pause      F1  F2  F3  F12   n           ],
        [t             t    t     t     t        n              n        t   t   t   t     t           ],
    }
};
