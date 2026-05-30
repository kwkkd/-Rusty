# AI Agent Operating System (AEOS) - Rust Implementation

AI 에이전트를 위한 운영체제를 Rust로 완전히 구현했습니다.

## 🎯 주요 기능

### 1. **커널 시스템** (`core/`)
- 에이전트 생명주기 관리
- 시스템 초기화 및 종료
- 업타임 추적

### 2. **에이전트 관리** (`agents/`)
- 에이전트 생성/삭제
- 상태 관리 (Idle, Running, Paused, Stopped, Failed)
- 메타데이터 저장

### 3. **작업 스케줄러** (`scheduler/`)
- 작업 생성 및 관리
- 작업 상태 추적
- 작업 실행 시간 기록

### 4. **리소스 관리** (`resources/`)
- CPU 사용률 모니터링
- 메모리 사용량 추적
- 시스템 정보 수집
- 리소스 가용성 확인

### 5. **메시지 버스** (`communication/`)
- 에이전트 간 메시지 전송
- 메시지 저장소
- 비동기 통신

### 6. **웹 인터페이스** (`ui/`)
- **Dashboard** - 실시간 시스템 모니터링
- **REST API** - 프로그래밍 인터페이스
  - 에이전트 관리 API
  - 작업 관리 API
  - 시스템 상태 API

### 7. **CLI 도구** (`bin/cli.rs`)
- `aeos-cli status` - 시스템 상태
- `aeos-cli agent` - 에이전트 관리
- `aeos-cli task` - 작업 관리
- `aeos-cli resources` - 리소스 모니터링
- `aeos-cli logs` - 로그 보기

## 🚀 시작하기

### 1. 빌드
```bash
cd aeos
cargo build --release
```

### 2. 실행
```bash
# 웹 서버 시작 (포트 8080)
cargo run --release

# 또는 빌드된 바이너리 실행
./target/release/aeos
```

### 3. 접근
- **Web Dashboard**: http://localhost:8080
- **API Base URL**: http://localhost:8080/api
- **CLI Tool**: `./target/release/aeos-cli`

## 📊 시스템 아키텍처

```
┌─────────────────────────────────────────────────┐
│          AEOS - Kernel System                   │
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌──────────────┬──────────────┬────────────┐  │
│  │    Agent     │     Task     │  Resource  │  │
│  │   Manager    │  Scheduler   │  Manager   │  │
│  └──────────────┴──────────────┴────────────┘  │
│           ↓           ↓            ↓            │
│  ┌──────────────────────────────────────────┐  │
│  │      Message Bus (Inter-Agent Comm)      │  │
│  └──────────────────────────────────────────┘  │
│           ↓           ↓            ↓            │
│  ┌──────────────┬──────────────┬────────────┐  │
│  │  Web API     │   CLI Tool   │ Dashboard  │  │
│  │  (Axum)      │              │ (Web UI)   │  │
│  └──────────────┴──────────────┴────────────┘  │
│                    ↓                           │
│  ┌──────────────────────────────────────────┐  │
│  │    Storage Layer (SQLite)                │  │
│  └──────────────────────────────────────────┘  │
│                                                 │
└─────────────────────────────────────────────────┘
```

## 🔌 API 엔드포인트

### 에이전트 API
```
GET    /api/agents              - 에이전트 목록
POST   /api/agents              - 에이전트 생성
GET    /api/agents/:id          - 에이전트 상세 정보
DELETE /api/agents/:id          - 에이전트 삭제
POST   /api/agents/:id/run      - 에이전트 실행
POST   /api/agents/:id/stop     - 에이전트 중지
```

### 작업 API
```
GET    /api/tasks               - 작업 목록
POST   /api/tasks               - 작업 생성
GET    /api/tasks/:id           - 작업 상세 정보
DELETE /api/tasks/:id           - 작업 삭제
```

### 시스템 API
```
GET    /api/system/status       - 시스템 상태
GET    /api/system/resources    - 리소스 사용량
GET    /api/system/logs         - 시스템 로그
```

## ⚙️ 환경 설정

`.env` 파일 또는 환경 변수로 설정:

```env
AEOS_PORT=8080              # 웹 서버 포트
AEOS_HOST=0.0.0.0           # 웹 서버 주소
DATABASE_URL=sqlite:./aeos.db  # 데이터베이스
RUST_LOG=debug              # 로그 레벨
AEOS_MAX_AGENTS=100         # 최대 에이전트 수
```

## 🛠️ 사용 예제

### CLI로 에이전트 관리
```bash
# 시스템 상태 확인
./aeos-cli status

# 에이전트 목록 조회
./aeos-cli agent list

# 새 에이전트 생성
./aeos-cli agent create "ProcessingAgent"

# 작업 생성
./aeos-cli task create "DataProcess" -a agent-001 -c "process data"

# 리소스 확인
./aeos-cli resources
```

### cURL로 API 사용
```bash
# 에이전트 생성
curl -X POST http://localhost:8080/api/agents \
  -H "Content-Type: application/json" \
  -d '{"name": "MyAgent"}'

# 작업 생성
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ProcessingTask",
    "agent_id": "agent-001",
    "command": "process --input data.txt"
  }'

# 시스템 상태 조회
curl http://localhost:8080/api/system/status
```

## 📦 핵심 의존성

- **tokio** - 비동기 런타임
- **axum** - 웹 프레임워크
- **sysinfo** - 시스템 정보 수집
- **sqlx** - 데이터베이스 접근
- **serde** - JSON 직렬화
- **uuid** - 고유 ID 생성
- **clap** - CLI 인자 파싱
- **colored** - 컬러 출력

## 🎓 프로젝트 구조

```
aeos/
├── src/
│   ├── main.rs           # 메인 진입점 (웹 서버)
│   ├── lib.rs            # 라이브러리 모듈
│   ├── bin/
│   │   └── cli.rs        # CLI 도구
│   ├── core/             # 커널 시스템
│   ├── agents/           # 에이전트 관리
│   ├── scheduler/        # 작업 스케줄링
│   ├── resources/        # 리소스 관리
│   ├── communication/    # 메시지 버스
│   ├── storage/          # 데이터베이스
│   ├── ui/               # 웹 UI 및 API
│   └── config/           # 설정 관리
├── Cargo.toml            # Rust 패키지 설정
├── README.md             # 프로젝트 설명서
└── PROGRESS.md           # 개발 진행 상황
```

## 🔄 다음 단계

1. ✅ 기본 구조 완성
2. ⏳ 데이터베이스 마이그레이션
3. ⏳ 고급 스케줄링 기능
4. ⏳ 플러그인 시스템
5. ⏳ 인증/권한 관리
6. ⏳ 컨테이너 배포

## 🐛 개발 및 테스트

```bash
# 빌드
cargo build --release

# 테스트
cargo test

# 코드 포맷
cargo fmt

# Lint 체크
cargo clippy

# 문서 생성
cargo doc --open
```

---

**AEOS v0.1.0** - Rust + Tokio로 빌드된 AI 에이전트 운영체제
