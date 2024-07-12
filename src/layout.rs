use keyberon::action::{k, m, Action::*, HoldTapAction, HoldTapConfig};
use keyberon::key_code::KeyCode::*;

type Action = keyberon::action::Action<()>;

static EXTRALAYER: Action = Action::DefaultLayer(3);
static QWERTZLAYER: Action = Action::DefaultLayer(0);

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

const TIMEOUT: u16 = 200;

const PPN: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(MediaNextSong),
    tap: k(MediaPlayPause),
});

// Level 0 - hold and tap

const A_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LShift),
    tap: k(A),
});

const S_CTL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(S),
});

const D_GUI: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LGui),
    tap: k(D),
});

const F_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(F),
});

const OE_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(RShift),
    tap: k(SColon), // Ö
});

const L_CTL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(RCtrl),
    tap: k(L),
});

const K_GUI: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LGui),
    tap: k(K),
});

const KB5_GUI: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LGui),
    tap: k(Kb5),
});

const J_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(J),
});

// Level 1 - hold and tap

const BS_CTL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: s!(Kb7),
});

const CCB_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: s!(Kb0),
});

const MI_CTL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Slash),
});

const OB_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: s!(Kb8),
});

// Level 2 - hold and tap

const HOME_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LShift),
    tap: k(Home),
});

const LEFT_CTL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Left),
});

const DOWN_GUI: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LGui),
    tap: k(Down),
});

const RIGHT_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(Right),
});


#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    {
        [ Tab    Q      W       E       R       T     Y   U       I       O       P       n      ]
        [ Escape {A_SH} {S_CTL} {D_GUI} {F_ALT} G     H   {J_ALT} {K_GUI} {L_CTL} {OE_SH} n      ]
        [ LShift Z      X       C       V       B     N   M       ,       .       Slash   RShift ]
        [                 n n n Escape  Enter  (2)   (1)  Space   Tab n n n                      ]
    }{
        [ t {a!(E)}     {s!(Grave)} {a!(Kb8)}         {a!(Kb9)}      Grave          {s!(Kb1)}   NonUsBslash {s!(NonUsBslash)} {s!(Kb0)}       {s!(Kb6)}   t ],
        [ t {a!(Minus)} {BS_CTL}    {a!(Kb7)}         {CCB_ALT}      {s!(RBracket)} {s!(Minus)} {OB_ALT}    {s!(Kb9)}         {MI_CTL}        {a!(Q)}     t ],
        [ t NonUsHash   {s!(Kb4)}   {a!(NonUsBslash)} {a!(RBracket)} {s!(Equal)}    RBracket    {s!(Kb5)}   {s!(Kb2)}         {s!(NonUsHash)} {s!(Comma)} t ],
        [ t t           t           t                 t              t              {EXTRALAYER} t          t                 t               t           t ],
    }{
        [ t  PgUp      Delete     Up         BSpace      PgDown         -        Kb7 Kb8       Kb9 Minus    t ], // Minsus = ß
        [ t  {HOME_SH} {LEFT_CTL} {DOWN_GUI} {RIGHT_ALT} End            Kb0      Kb4 {KB5_GUI} Kb6 LBracket t ], // LBracket = Ü
        [ t  LGui      Tab        Mute       Enter       CapsLock       RBracket Kb1 Kb2       Kb3 Quote    t ], // Quote = Ä ; RBracket = +
        [ t  t         t          t          t           {EXTRALAYER}   t        t   t         t   t        t ],
    }{
        [{Custom(())}  n    n     {PPN} VolUp    n             PScreen    F7  F8  F9  F10  {Custom(())}],
        [{QWERTZLAYER} n    n     n     Mute     n            ScrollLock F4  F5  F6   F11  t           ],
        [n             A    n     n     VolDown  n            Pause      F1  F2  F3  F12   n           ],
        [t             t    t     t     t        n              n        t   t   t   t     t           ],
    }
};
