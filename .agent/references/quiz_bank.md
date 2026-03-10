# 복습 퀴즈 뱅크

> 사용법: 새 세션 시작 시 logs.md에서 최근 학습 주제를 확인하고,
> 해당 섹션에서 퀴즈 1~2개를 골라 출제한다.

---

## 해시맵 재도전 & 빌림 검사기 (2026-03-03)

**Q1.** 아래 코드에서 `team_1` 변수가 담고 있는 '값'의 정체는 무엇인가요? 메모리 주소(참조)의 관점에서 설명해보세요.
```rust
let team_1 = scores.entry("England").or_default();
```

**Q2.** 아래 코드에서 `team_1`의 필드를 수정했을 뿐인데, 왜 `scores.insert()`를 다시 호출하지 않아도 `scores` 전체가 업데이트된 상태가 되나요?
```rust
let team_1 = scores.entry("England").or_default();
team_1.goals_scored += 3;
// scores.insert("England", team_1); <- 왜 이 코드가 필요 없을까요?
```

**Q3.** 아래 코드는 컴파일 에러가 발생합니다. 왜 한 번에 하나씩만 `entry()`를 통해 가변 참조를 만들 수 있는지, Rust의 '빌림 규칙'과 연결해서 설명해보세요.
```rust
let team_1 = scores.entry("England").or_default();
let team_2 = scores.entry("France").or_default(); // ERROR!
team_1.goals_scored += 1;
```

---

## Default 트레이트 & Entry API 심화 (2026-02-27)

**Q1.** 아래와 같이 `or_default()`를 사용해 해시맵 값을 생성하려면 `TeamScores` 구조체에 어떤 트레이트가 구현되어 있어야 할까요?
```rust
let score = scores.entry("RedTeam").or_default();
```

**Q2.** 아래 두 코드의 결정적인 차이점(어떤 값을 넣는가)은 무엇인가요?
```rust
scores.entry("A").or_default();
scores.entry("B").or_insert(TeamScores { goals: 10 });
```

**Q3.** `entry().or_default()`가 반환하는 값의 정체는 무엇인가요? (값 자체인가요, 아니면 참조인가요?)

---

## 소유권 & 이동 시맨틱 (2026-02-20)

**Q1.** 아래 코드는 컴파일될까요? 왜 그럴까요?
```rust
let s = String::from("hello");
let s2 = s;
println!("{}", s);
```

**Q2.** `&`와 `&mut`의 차이를 "동시에 몇 명이 쓸 수 있는가"로 설명해보세요.

**Q3.** 함수 인자로 아래와 같이 `mut`을 붙여서 전달받으면 무엇이 가능해지나요?
```rust
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> { ... }
```

---

## 타입 심화 String vs &str (2026-02-19 / 2026-02-23)

**Q1.** 아래 두 함수 중 더 유연하고 권장되는 설계는 무엇인가요? 그 이유는?
```rust
fn takes_string(s: String) { ... }
fn takes_slice(s: &str) { ... }
```

**Q2.** `to_string()`, `to_owned()`, `String::from()` 세 가지 중 기능적으로 동일한 두 가지는?

**Q3.** `to_lowercase()`가 `&str` 대신 `String`을 반환하는 이유는?

---

## 구조체 (2026-02-21)

**Q1.** Rust 구조체의 세 가지 형태(일반, 튜플, 유닛)를 각각 한 줄 코드로 선언해보세요.

**Q2.** `#[derive(Debug)]`와 `{:?}`는 각각 무슨 역할을 하나요?

**Q3.** 아래와 같이 구조체 업데이트 구문(`..`)을 쓸 때, `p1`의 소유권 이슈가 생기는 필드는 무엇인가요?
```rust
struct Person { name: String, age: u32 }
let p1 = Person { name: String::from("Kim"), age: 20 };
let p2 = Person { age: 21, ..p1 };
```

---

## 열거형 (2026-02-21 / 2026-02-22)

**Q1.** 구조체(Struct)와 열거형(Enum)의 차이를 "AND / OR"로 설명해보세요.

**Q2.** 아래 코드에서 `match` 없이 `msg.x` 값에 바로 접근할 수 없는 이유는?
```rust
enum Message { Move { x: i32, y: i32 }, Quit }
```

**Q3.** `Command::Move { x, y }` 처럼 명령을 데이터화하면 직접 함수 호출 대비 어떤 이점이 생기나요?

---

## 모듈 & 가시성 (2026-02-23)

**Q1.** Rust에서 모든 항목의 기본 가시성은 무엇인가요?

**Q2.** `pub use`(re-exporting)는 어떤 상황에서 유용한가요?

**Q3.** `use std::time::{SystemTime, UNIX_EPOCH};` 처럼 쓰는 문법의 이름은?

---

## HashMap 기초 (2026-02-24)

**Q1.** HashMap을 쓰려면 코드 상단에 무엇을 선언해야 하나요?

**Q2.** 테스트 코드 안에 `println!`을 써도 출력이 안 되는 이유는? 출력하려면?

---

## HashMap 심화 & Entry API (2026-02-26)

**Q1.** `.entry(key).or_default()`에서 `or_default()`가 기본값을 만들려면 구조체에 무엇이 구현되어 있어야 하나요?

**Q2.** 아래 코드를 실행했을 때 실제로 연산이 일어날까요? Iterator의 '지연 평가' 특성과 연결해 설명해보세요.
```rust
let iter = vec![1, 2, 3].iter().map(|x| x + 1);
```

**Q3.** `next().unwrap().parse::<u8>().unwrap()` 에서 `parse()`가 어떤 타입으로 변환할지 아는 방법은?

---

## 기초 문법 (2026-02-18)

**Q1.** Rust에서 `if`가 함수와 다르게 특별한 이유는?

**Q2.** 매크로 이름 뒤에 `!`를 붙이는 이유(의의)는?

**Q3.** `char` 타입은 몇 바이트이며, 왜 그 크기인가요?
