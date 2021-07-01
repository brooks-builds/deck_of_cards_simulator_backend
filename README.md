deck_of_cards_simulator_backend

```rust
struct MainState {
    rooms: Vec<Room>
}

struct Room {
    id: String
    draw_deck: Vec<Card>
    players: Vec<Player>
}

struct Card {
    id: u8,
    visible: bool,
    suite: CardSuite,
    value: CardValue,
}

struct Player {
    name: String,
    hand: Vec<Card>,
    sender: UnboundedSender<Tungstenite Message>,
    address: String
}

enum CardSuite {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
    Hidden,
}

enum CardValue {
    Two,
    Three,
    Four,
    ...,
    Hidden,
}

enum Action {
    CreateGame,
    JoinGame,
    DrawCard,
    ToggleCardVisibility,
    CollectCards,
}

struct CustomMessage {
    action: Action,
    data: MessageData
}

struct MessageData {
    player_name: Option<String>,
    room_id: Option<String>,
    card: Option<Card>,
    card_id: Option<u8>,
    player_id: Option<String>
    draw_deck_size: Option<u8>
}
```
