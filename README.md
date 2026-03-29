Đây là bản thảo file **README.md** chuyên nghiệp, được thiết kế chuẩn cấu trúc GitHub dành cho dự án của bạn. Tôi đã tích hợp nội dung về ý tưởng **Thanh Toán Gia Sư Đồng Nghiệp** bằng Rust (Soroban) mà chúng ta đã thảo luận.

---

# 🎓 EDU-REWARD NETWORK (ERN)
### *Hệ thống thanh toán gia sư đồng nghiệp trên nền tảng Stellar Soroban*

---

## 📝 Mô tả dự án (Description)
**Edu-Reward Network (ERN)** là một nền tảng dApp phi tập trung cho phép sinh viên tìm kiếm và thanh toán cho các buổi học gia sư ngắn hạn (Peer-to-Tutoring) một cách tức thì và an toàn.

* **Mục đích:** Giải quyết vấn đề tin cậy giữa sinh viên với nhau và loại bỏ phí giao dịch ngân hàng phi lý khi thanh toán các khoản nhỏ (micro-payments).
* **Tại sao làm ý tưởng này?** Là một sinh viên đang học **Pedagogy** và **Computer Science**, tôi nhận thấy nhu cầu học nhóm và trao đổi kiến thức là rất lớn. Việc sử dụng Smart Contract giúp đảm bảo: **Dạy xong mới mất tiền - Học xong là có tiền.**

## ✨ Tính năng (Features)
* **Escrow Payment (Hợp đồng giữ hộ):** Tiền học phí được khóa trong Smart Contract ngay khi buổi học bắt đầu.
* **Instant Disbursement (Giải ngân tức thì):** Ngay khi người học xác nhận "Complete", tiền sẽ được chuyển thẳng đến ví gia sư trong 3-5 giây.
* **University Token (UNIT):** Hỗ trợ thanh toán bằng Token nội bộ của trường hoặc các Stablecoin như USDC trên mạng Stellar.
* **Phí cực thấp:** Tận dụng mạng lưới Stellar để giao dịch với chi phí gần như bằng 0 (0.00001 XLM).

## ⚙️ Hợp đồng thông minh (Contract)
https://stellar.expert/explorer/testnet/contract/CAY5RKDJD3WOI7XSPTFFPO6IOW7V4BFTDVCICLWPRTS7RCQGHWZG5Q7U
> **Ảnh chụp màn hình Contract:** > *(Bạn hãy đính kèm ảnh chụp màn hình từ Stellar Expert hoặc terminal khi invoke thành công tại đây)*
<img width="1919" height="1075" alt="image" src="https://github.com/user-attachments/assets/67581288-d85f-4e14-ac2c-4720e8844611" />

## 🚀 Tầm nhìn tương lai (Future Scopes)
* **Hệ thống Rating:** Tích hợp điểm uy tín cho gia sư trực tiếp vào Blockchain để sinh viên dễ dàng chọn lựa người dạy giỏi.
* **Tự động hóa lịch trình:** Tích hợp lịch học, nếu gia sư không xuất hiện, tiền sẽ tự động hoàn trả (Refund) cho sinh viên.
* **Mở rộng quy mô:** Áp dụng cho toàn bộ các trường đại học trong khối ĐH Đà Nẵng, tạo thành một hệ sinh thái trao đổi tri thức số.

## 👤 Thông tin cá nhân (Profile)
* **Nickname:** `studen` / [Tên của bạn]
* **Trường:** University of Economics – University of Da Nang (DUE)
* **Kỹ năng:** * **Ngôn ngữ:** Rust (Soroban), Python.
    * **Chuyên môn:** Pedagogy, Marx-Leninist Philosophy, Computer Architecture.
* **Mục tiêu:** Trở thành kỹ sư Blockchain có tư duy sư phạm để lan tỏa công nghệ đến cộng đồng sinh viên.

---

### Cách chạy thử nghiệm (Quick Start)
Để thử nghiệm hàm chào hỏi trên contract này, hãy dùng lệnh:
```bash
stellar contract invoke --id CAY5RKDJD3WOI7XSPTFfPO6IOW7V4BFTDVCICLWPRTS7RCQGHWZG5Q7U --source-account studen --network testnet -- hello --to "[Tên_Của_Bạn]"
```

---
**Bạn có muốn tôi điều chỉnh thêm phần "Kỹ năng" hay thêm mục "Hướng dẫn cài đặt" (Installation) vào README này không?**
