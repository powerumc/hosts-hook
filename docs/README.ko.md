# 🔄 hosts-hook [[English](../README.md)]

**hosts-hook**은 시스템의 `hosts` 파일을 수정하지 않고 특정 호스트 이름에 대한 DNS 조회를 재정의(override)할 수 있는 도구입니다. 이 기능은 로컬이나 다른 서버로 트래픽을 리디렉션해야 하는 개발 및 테스트 환경에서 특히 유용합니다.

## ✨ 기능

- **동적 DNS 재정의**: 시스템의 DNS 조회를 가로채고 사용자 정의 응답을 제공합니다.
- **유연한 설정**: 현재 디렉토리와 상위 디렉토리에서 사용자 정의 hosts 파일(`.hosts` 또는 `hosts`)을 검색합니다.
- **환경별 Hosts 파일**: `HOSTS_ENV` 변수를 사용하여 개발, 스테이징 등 다양한 환경에 맞는 hosts 파일을 사용할 수 있습니다.
- **IPv4 & IPv6 지원**: IPv4와 IPv6 주소를 모두 지원합니다.
- **쉬운 사용법**: 간단한 명령어로 설정을 초기화하고 후킹을 활성화할 수 있습니다.

## 💻 지원 플랫폼

- **macOS**
- **Linux**

## 📥 설치

먼저 시스템에 [Rust](https://www.rust-lang.org/tools/install)가 설치되어 있어야 합니다.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 🛠️ 수동 빌드 (현재 사용 가능한 방법)

저장소를 클론하고 Cargo를 사용하여 프로젝트를 빌드합니다:

```bash
# 저장소 클론
git clone https://github.com/powerumc/hosts-hook.git
cd hosts-hook

# 프로젝트 빌드
cargo build
```

### macOS (Homebrew)

```bash
brew tap powerumc/tap

brew install hostshook
```

## 📚 사용 방법

`hosts-hook`은 hosts 파일을 관리하는 CLI 도구와 DNS 재정의를 활성화하는 후킹 메커니즘, 두 가지 주요 부분으로 구성됩니다.

### 1. Hosts 파일 초기화하기

`hostshook init` 명령을 사용하여 현재 디렉토리에 `.hosts` 파일을 생성합니다. 이 파일에는 사용자 정의 DNS 규칙이 포함됩니다.

```bash
# 기본 .hosts 파일 생성
hostshook init

# 특정 환경을 위한 이름의 hosts 파일 생성
hostshook init --file .hosts.development
```

### 2. DNS 후킹 활성화하기

DNS 재정의를 활성화하려면 `hosts-hook` 동적 라이브러리를 셸 세션에 로드해야 합니다. `hostshook` 명령을 사용하면 이 과정을 쉽게 수행할 수 있습니다.

```bash
# 이 명령어는 필요한 export 구문을 출력합니다.
# `source` 명령어는 현재 셸에서 이 구문을 실행합니다.
source <(hostshook)

# 또는

hostshook
# 다음과 유사한 내용이 출력됩니다:
# export DYLD_INSERT_LIBRARIES=/path/to/your/libhostshook.dylib
```
활성화되면 이 셸에서 실행되는 모든 새로운 프로세스는 `hosts-hook`에 의해 DNS 조회가 가로채집니다.

### 3. 다른 Hosts 파일을 위한 환경 설정

`HOSTS_ENV` 환경 변수를 설정하여 다른 hosts 설정을 사용할 수 있습니다.

```bash
# 환경을 'development'로 설정
export HOSTS_ENV=development

# 이제 hosts-hook은 .hosts.development 또는 hosts.development 파일을 찾습니다.
```

## ⚙️ 작동 방식

**hosts-hook**은 DNS 해석과 관련된 시스템 호출(`getaddrinfo` 등)을 가로채는 방식으로 작동합니다. 애플리케이션이 도메인 이름 해석을 시도할 때, 후킹된 함수는 먼저 사용자 정의 `.hosts` 파일에서 일치하는 항목이 있는지 확인합니다.

일치하는 항목을 찾으면, 표준 DNS 해석 프로세스를 우회하고 사용자가 지정한 IP 주소를 반환합니다. 일치하는 항목이 없으면, 요청은 시스템의 기본 DNS 해석기로 전달됩니다.

hosts 파일 검색은 현재 디렉토리에서 시작하여 루트 디렉토리까지 상위로 이동하며 이루어집니다.

## 🗂️ Hosts 파일 검색 순서

`hosts-hook`은 다음 우선순위에 따라 hosts 파일을 검색합니다:

1.  `hosts.<environment>` (예: `HOSTS_ENV=development`일 경우 `hosts.development`)
2.  `.hosts.<environment>` (예: `.hosts.development`)
3.  `hosts`
4.  `.hosts`

검색은 현재 디렉토리에서 먼저 수행된 후, 파일을 찾을 때까지 루트 디렉토리 방향으로 상위 디렉토리를 재귀적으로 탐색합니다.

## 📄 Hosts 파일 형식

파일은 표준 hosts 형식을 따릅니다. `#`으로 시작하는 줄은 주석입니다.

```
# 이것은 주석입니다
127.0.0.1 example.com
::1       ipv6.example.com
```

## 🧪 예제

저장소의 `examples` 디렉토리에서 **C** 및 **Node.js** 예제를 확인할 수 있습니다.

### Node.js 예제 실행하기

```bash
cd examples/nodejs
./test.sh
```

## ⚠️ 제한사항

### macOS SIP (시스템 무결성 보호)

macOS에서는 시스템 무결성 보호(SIP) 기능으로 인해 `ping`, `curl`, `ssh`와 같은 시스템 수준의 명령어 및 애플리케이션은 후킹되지 않습니다. 이는 핵심 시스템 구성 요소를 보호하기 위한 OS의 보안 기능입니다.

따라서 이러한 보호된 명령어들은 `hosts-hook`을 무시하고 표준 시스템 DNS 해석기를 사용합니다.

하지만 **대부분의 서드파티 애플리케이션은 SIP의 영향을 받지 않습니다**. 여기에는 다음이 포함됩니다:
- 개발 도구 (Node.js, Python, Java 애플리케이션)
- 데이터베이스 클라이언트
- 그 외 다수.

이러한 애플리케이션들은 `hosts-hook`이 제공하는 DNS 재정의를 올바르게 사용합니다.

## 📜 라이선스

이 프로젝트는 **MIT 라이선스** 하에 배포됩니다.

## 👥 기여

이슈와 풀 리퀘스트는 환영합니다! 🙏
