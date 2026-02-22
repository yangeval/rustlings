// 🏠 Smart Home Device Lab: Smart Light Controller
// enums3 문제와 비슷하게, 다양한 명령어(Enum)를 받아 상태(Struct)를 변경하는 로직을 작성해보세요.

struct SmartLight {
    is_on: bool,
    brightness: u8, // 0-100
    color_temp: u32, // 2000-6500 (Kelvin)
    mode: String,
}

enum LightCommand {
    // TODO: 아래 테스터에서 사용되는 명령어들을 정의하세요.
    // 1. Off (데이터 없음)
    // 2. Brightness (u8 데이터를 가짐)
    // 3. ColorTemp (u32 데이터를 가짐)
    // 4. ChangeMode (String 데이터를 구조체 스타일 { mode: String } 로 가짐)
    Off,
    SetBrightness(u8),
    SetColorTemp(u32),
    ChangeMode{ mode:String }
}

impl SmartLight {
    fn new() -> Self {
        Self {
            is_on: true,
            brightness: 50,
            color_temp: 4000,
            mode: String::from("Normal"),
        }
    }

    fn set_off(&mut self) {
        self.is_on = false;
    }

    fn set_brightness(&mut self, b: u8) {
        self.brightness = b;
    }

    fn set_color_temp(&mut self, temp: u32) {
        self.color_temp = temp;
    }

    fn change_mode(&mut self, mode: String) {
        self.mode = mode;
    }

    // TODO: handle_command 메서드를 완성하세요.
    // match 문을 사용하여 들어온 command에 따라 위의 메서드들을 호출해야 합니다.
    fn handle_command(&mut self, command: LightCommand) {
        // 여기에 코드를 작성하세요.
        match command {
            LightCommand::Off => {
                self.set_off()
            }
            LightCommand::SetBrightness(b) => {
                self.set_brightness(b)
            }
            LightCommand::SetColorTemp(temp) => {
                self.set_color_temp(temp)
            }
            LightCommand::ChangeMode{mode} => {
                self.change_mode(mode)
            }
           
        }
    }
}

fn main() {
    // 실험해보고 싶다면 여기에 코드를 작성하세요.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_commands() {
        let mut light = SmartLight::new();

        // 1. 모드 변경 테스트
        light.handle_command(LightCommand::ChangeMode {
            mode: String::from("Sleep"),
        });
        assert_eq!(light.mode, "Sleep");

        // 2. 밝기 변경 테스트
        light.handle_command(LightCommand::SetBrightness(10));
        assert_eq!(light.brightness, 10);

        // 3. 색온도 변경 테스트
        light.handle_command(LightCommand::SetColorTemp(3000));
        assert_eq!(light.color_temp, 3000);

        // 4. 전원 끄기 테스트
        light.handle_command(LightCommand::Off);
        assert_eq!(light.is_on, false);
    }
}
