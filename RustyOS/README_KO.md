# RustyOS - 실제 운영체제 🖥️

**Rust로 만든 x86-64 아키텍처 기반의 완전한 운영체제입니다.**

## 🎯 프로젝트 개요

RustyOS는 다음을 보여주는 교육용이면서 동시에 작동하는 운영체제입니다:
- 커스텀 부트로더 통합
- x86-64 커널 개발
- 메모리 관리 (페이징, 힙 할당)
- 인터럽트 처리 (IDT, GDT, PIC)
- 키보드 및 시리얼 입력
- VGA 텍스트 모드 디스플레이
- 작업 스케줄링 기초

## ✨ 주요 기능

### 핵심 구성요소
- **부트로더** - `bootloader 0.9` 크레이트를 사용한 UEFI 부팅
- **메모리 관리**
  - 페이징을 통한 가상 메모리
  - 범프 할당기를 사용한 힙 할당
  - 물리 메모리 프레임 할당기
- **CPU 관리**
  - 전역 설명자 테이블 (GDT)
  - 인터럽트 설명자 테이블 (IDT)
  - 프로그래머블 인터럽트 컨트롤러 (PIC8259)
- **I/O 서브시스템**
  - 시리얼 포트 통신 (UART 16550)
  - VGA 텍스트 모드 (80x25)
  - 키보드 입력 처리
- **인터럽트 및 예외**
  - 이중 오류 처리
  - 페이지 오류 처리
  - 타이머 인터럽트
  - 키보드 인터럽트

## 🚀 빠른 시작

### 사전 요구사항
- Rust 1.70+ with `nightly` 도구체인
- QEMU (테스트용)
- bootimage 도구

### 설치

```bash
# 프로젝트 열기
cd RustyOS

# Rust nightly 설치 (필요시)
rustup toolchain install nightly
rustup component add llvm-tools-embedded

# bootimage 설치
cargo install bootimage
```

### 빌드

```bash
# 부팅 가능한 이미지 생성
make build

# 또는 cargo 직접 사용
cargo build --release
bootimage build --release
```

### 실행

```bash
# QEMU에서 실행
make run

# KVM 가속으로 실행
make kvm

# 수동 QEMU 호출
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin
```

## 📁 프로젝트 구조

```
RustyOS/
├── src/
│   ├── main.rs           # 커널 진입점
│   ├── gdt.rs            # 전역 설명자 테이블 설정
│   ├── interrupts.rs     # 인터럽트 처리 (IDT, PIC)
│   ├── memory.rs         # 메모리 관리 및 할당기
│   ├── vga.rs            # VGA 텍스트 모드 드라이버
│   ├── serial.rs         # 시리얼 포트 통신
│   ├── keyboard.rs       # 키보드 입력 처리
│   ├── allocator.rs      # 힙 할당기
│   └── task.rs           # 작업 스케줄링
├── .cargo/config.toml    # Cargo 빌드 설정
├── Cargo.toml            # 프로젝트 의존성
├── Makefile              # 빌드 자동화
├── build.sh              # 빌드 스크립트
└── README.md             # 이 파일
```

## 🛠️ 개발

### 빌드 변종
```bash
# 디버그 빌드
cargo build --target x86_64-unknown-none

# 최적화된 릴리스 빌드
cargo build --release

# 빌드 없이 코드 확인
cargo check
```

### 테스트
```bash
# 단위 테스트 실행
cargo test --lib

# 통합 테스트 (tests/ 디렉토리)
make test
```

### 디버깅
```bash
# GDB 지원으로 실행
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/release/boot-image.bin \
                   -s -S &
# 다른 터미널에서:
gdb -ex "file target/x86_64-unknown-none/release/kernel" \
    -ex "target remote :1234"
```

## 📊 시스템 사양

| 구성요소 | 명세 |
|---------|-----|
| **아키텍처** | x86-64 (Intel/AMD 호환) |
| **부트 모드** | Bootloader 0.9를 통한 UEFI |
| **메모리** | 페이징 활성화, 힙 할당 |
| **디스플레이** | VGA 텍스트 모드 (80x25) |
| **인터럽트** | 256개 항목을 가진 IDT |
| **터미널** | 시리얼 포트 (UART 16550) |
| **키보드** | PS/2 키보드 지원 |

## 🎮 키보드 제어

| 키 | 기능 |
|----|------|
| `Ctrl+C` | 중단 (지원 시) |
| `Enter` | 줄 바꿈 |
| `Backspace` | 문자 삭제 |

## 📦 의존성

### 빌드 의존성
- `bootloader` v0.9 - 부트로더 통합
- `x86_64` v0.14 - x86-64 아키텍처 지원
- `volatile` v0.4 - 휘발성 메모리 접근
- `uart_16550` v0.2 - 시리얼 포트 드라이버
- `pic8259` v0.10 - 프로그래머블 인터럽트 컨트롤러
- `pc-keyboard` v0.7 - 키보드 레이아웃 처리
- `lazy_static` v1.4 - 정적 초기화
- `spin` v0.9 - 스핀락 구현
- `linked-list-allocator` v0.10 - 힙 할당기

## 🔧 설정

### 환경 변수
```bash
# 로그 레벨 설정 (debug, info, warn, error)
RUST_LOG=debug

# 커스텀 빌드 대상
RUST_TARGET=x86_64-unknown-none
```

### 빌드 설정 (.cargo/config.toml)
- 대상: `x86_64-unknown-none`
- 링커: LLD (bootimage를 통해)
- 링커 플래그: 표준 시작 파일 없음

## 📖 학습 자료

- [OSDev.org](https://wiki.osdev.org/) - 운영체제 개발 위키
- [Rust로 OS 작성](https://os.phil-opp.com/) - 우수한 튜토리얼
- [x86-64 문서](https://en.wikipedia.org/wiki/X86-64) - 아키텍처 참조
- [Intel 64 매뉴얼](https://www.intel.com/) - 공식 명세

## 🐛 문제 해결

### 빌드 실패
```bash
# Rust 도구체인 업데이트
rustup update
rustup component add llvm-tools-embedded

# 깨끗하게 재빌드
cargo clean
cargo build --release
```

### QEMU를 찾을 수 없음
```bash
# QEMU 설치
# Ubuntu/Debian
sudo apt-get install qemu-system-x86

# macOS
brew install qemu

# Windows
choco install qemu
```

### Bootimage 오류
```bash
# bootimage 재설치
cargo install bootimage --force
```

## 🚧 로드맵

### 단계 1: ✅ 핵심 커널
- [x] UEFI 부트로더에서 부팅
- [x] 기본 인터럽트 처리
- [x] 메모리 관리
- [x] VGA 출력
- [x] 시리얼 통신

### 단계 2: ⏳ 고급 기능
- [ ] 파일시스템 (FAT32/ext2)
- [ ] 멀티태스킹/스케줄링
- [ ] 네트워크 스택
- [ ] 장치 드라이버 (ATA, NVMe)
- [ ] 사용자 모드 실행
- [ ] 시스템 호출

### 단계 3: 🔮 시스템 서비스
- [ ] 셸/명령어 해석기
- [ ] 파일 유틸리티
- [ ] 시스템 유틸리티
- [ ] 패키지 관리자
- [ ] GUI/윈도우 관리자
- [ ] 컴파일러 통합

## 📝 라이선스

이 프로젝트는 교육 목적으로 제공됩니다.

## 🤝 기여

기여를 환영합니다! 기여 영역:
- 드라이버 개발
- 파일시스템 구현
- 스케줄러 개선
- 문서화
- 테스팅 및 버그 수정

## 📞 지원

질문 및 논의:
- 저장소에서 이슈 열기
- 기존 문서 확인
- OSDev.org에서 OS 개념 검토

---

**RustyOS v0.1.0** - ❤️로 만든 Rust | x86-64용 설계 | 교육용 OS 프로젝트

```
╔════════════════════════════════════════════════════════╗
║                   🖥️ RustyOS v0.1.0                    ║
║         Rust로 작성된 현대식 운영체제                    ║
╚════════════════════════════════════════════════════════╝
```
