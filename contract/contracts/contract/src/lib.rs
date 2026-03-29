#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, token};

#[contract]
pub struct EduRewardContract;

#[contractimpl]
impl EduRewardContract {
    /// Hàm khởi tạo buổi học và giữ tiền (Escrow)
    pub fn create_session(
        env: Env,
        student: Address,
        tutor: Address,
        token_address: Address,
        amount: i128,
    ) {
        // 1. Xác thực quyền sở hữu của sinh viên
        student.require_auth();

        // 2. Khởi tạo client cho Token (ví dụ: UNIT token của trường)
        let token_client = token::Client::new(&env, &token_address);

        // 3. Chuyển tiền từ sinh viên vào Contract này để giữ hộ
        token_client.transfer(&student, &env.current_contract_address(), &amount);

        // 4. Lưu trữ thông tin buổi học vào Storage của Contract
        // Trong thực tế, bạn nên dùng một Struct và Mapping ID buổi học
        env.storage().instance().set(&tutor, &amount);
    }

    /// Hàm xác nhận hoàn thành và trả tiền cho gia sư
    pub fn complete_session(
        env: Env, 
        student: Address, 
        tutor: Address,
        token_address: Address
    ) {
        // Chỉ sinh viên mới có quyền xác nhận giải ngân
        student.require_auth();

        // Lấy số tiền đang tạm giữ cho gia sư này
        let amount: i128 = env.storage().instance().get(&tutor).unwrap_or(0);
        
        if amount > 0 {
            let token_client = token::Client::new(&env, &token_address);
            
            // Chuyển tiền từ Contract cho Gia sư
            token_client.transfer(&env.current_contract_address(), &tutor, &amount);
            
            // Xóa dữ liệu tạm giữ sau khi đã thanh toán
            env.storage().instance().remove(&tutor);
        }
    }
}

stellar contract invoke --id CAY5RKDJD3WOI7XSPTFFPO6IOW7V4BFTDVCICLWPRTS7RCQGHWZG5Q7U --source-account student --network testnet -- create_session --student student --tutor student1 --token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC --amount 50