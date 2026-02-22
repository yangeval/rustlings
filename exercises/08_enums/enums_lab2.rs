struct Character {
    health: u32,
    x: i32,
    y: i32,
    inventory: Vec<String>,
}

// 0단계: 메시지 체계(리모컨 버튼)를 직접 설계해보세요!
enum GameCommand {
    // 여기에 정의하세요 (Move, Attack, Heal, PickUp)
    SetMove { x: i32, y: i32 },
    DoAttack(u32),
    DoHeal(u32),
    PickUp(String)
}

impl Character {
    fn new() -> Self {
        Self {
            health: 100,
            x: 0,
            y: 0,
            inventory: vec![],
        }
    }

    fn set_move(&mut self, x:i32, y:i32){
        self.x = x;
        self.y = y;
    }
    fn do_attack(&mut self, damage_value:u32){
        self.health = self.health - damage_value;
    }
    fn do_heal(&mut self, heal_value:u32){
        self.health = self.health + heal_value;
    }
    fn pick_up(&mut self, item:String){
        self.inventory.push(item);
    }

    // 1단계: 명령을 받아 행동하는 지휘 본부를 만드세요.
    fn process_command(&mut self, command: GameCommand) {
        // match 문을 작성하세요
        match command {
            GameCommand::SetMove{x, y} => {
                self.set_move(x, y)
            }
            GameCommand::DoAttack(damage_value) => {
                self.do_attack(damage_value)
            }
            GameCommand::DoHeal(heal_value) => {
                self.do_heal(heal_value)
            }
            GameCommand::PickUp(item) => {
                self.pick_up(item)
            }
        }
    }
}

fn main() {
    let mut hero = Character::new();

    // 2단계: 직접 명령을 내려보세요!
    // hero.process_command(...)
    hero.process_command(GameCommand::SetMove{x:4, y:10});
    hero.process_command(GameCommand::DoAttack(20));
    hero.process_command(GameCommand::DoHeal(10));
    hero.process_command(GameCommand::PickUp(String::from("sword")));

    println!("캐릭터 상태: 체력 {}, 위치({}, {}), 소지품: {:?}", 
        hero.health, hero.x, hero.y, hero.inventory);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_action() {
        let mut hero = Character::new();
        
        // 3단계: 여기에 원하는 테스트 코드를 작성해서 


        // hero의 상태가 잘 변하는지 assert_eq! 로 확인해보세요.
    }
}
