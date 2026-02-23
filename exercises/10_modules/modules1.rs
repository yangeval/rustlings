// TODO: Fix the compiler error about calling a private function.
// TODO: 비공개(private) 함수 호출과 관련된 컴파일 에러를 수정하세요.
// 
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
