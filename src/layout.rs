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

//  Thumb keys

const ESC_CTRL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Escape),
});

const ENTER_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(Enter),
});

const SPACE_ALT: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LAlt),
    tap: k(Space),
});

const TAB_CTRL: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LCtrl),
    tap: k(Tab),
});


// Level 0 - hold and tap

const Z_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LShift),
    tap: k(Z),
});

const S_GUI: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LGui),
    tap: k(S),
});

const L_GUI: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LGui),
    tap: k(L),
});

const MINUS_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(RShift),
    tap: k(Slash),
});

const A_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LShift),
    tap: k(A),
});

const OE_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(RShift),
    tap: k(SColon), // scolon = ö
});

// Level 1 - hold and tap

const HOME_SH: Action = HoldTap(&HoldTapAction {
    timeout: TIMEOUT,
    tap_hold_interval: 200,
    config: HoldTapConfig::Default,
    hold: k(LShift),
    tap: k(Home),
});

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<12, 4, 4, ()> = keyberon::layout::layout! {
    {
        [ n Q      W       E       R       T     Y   U       I       O       P          n ]
        [ n {A_SH} {S_GUI} D       F       G     H   J       K       {L_GUI} {OE_SH}    n ]
        [ n {Z_SH} X       C       V       B     N   M       ,       .       {MINUS_SH} n ]
        [        n n n {ESC_CTRL} {ENTER_ALT}  (2)   (1) {SPACE_ALT} {TAB_CTRL}  n n n    ]
    }{
        [ t {a!(E)}     {s!(Grave)} {a!(Kb8)}         {a!(Kb9)}      Grave          {s!(Kb1)}   NonUsBslash {s!(NonUsBslash)} {s!(Kb0)}       {s!(Kb6)}   t ],
        [ t {a!(Minus)} {s!(Kb7)}   {a!(Kb7)}         {a!(Kb0)}      {s!(RBracket)} {s!(Minus)} {s!(Kb8)}   {s!(Kb9)}         Slash           {a!(Q)}     t ],
        [ t NonUsHash   {s!(Kb4)}   {a!(NonUsBslash)} {a!(RBracket)} {s!(Equal)}    RBracket    {s!(Kb5)}   {s!(Kb2)}         {s!(NonUsHash)} {s!(Comma)} t ],
        [ t t           t           t                 t              t              {EXTRALAYER} t    t                t               t           t ],
    }{
        [ t  PgUp      Delete     Up         BSpace      PgDown         LGui     Kb7 Kb8 Kb9 Minus    t ], // Minus = ß ; Slash = Minus
        [ t  {HOME_SH} Left       Down       Right       End            Kb0      Kb4 Kb5 Kb6 LBracket t ], // LBracket = Ü
        [ t  LShift    LGui       Space      Enter       CapsLock       RBracket Kb1 Kb2 Kb3 Quote    t ], // Quote = Ä ; RBracket = +
        [ t  t         t          t          t       {EXTRALAYER}       t        t   t   t   t        t ],
    }{
        [n  n    {PPN}  VolUp    n    n            PScreen    F7  F8  F9  F10   t ],
        [n  n    LCtrl  Mute     LAlt n            ScrollLock F4  F5  F6   F11  t ],
        [n  A    n      VolDown  n    n            Pause      F1  F2  F3  F12   t ],
        [t  t    t   t  {QWERTZLAYER} n            n  {QWERTZLAYER}   t   t   T t ],
    }
};
