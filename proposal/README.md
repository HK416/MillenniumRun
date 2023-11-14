# Millennium Run

[한국어](#기획서-한국어) </br>
[English (translation)](#proposal-english-translation) </br>

# 기획서 (한국어)
## 1. 스토리
밀레니엄 학원의 게임 개발부가 자신들이 만든 게임을 플레이해보고 감상을 들려달라는 부탁을 받은 샬레의 선생이 게임 개발부 부실에 방문한다. '사이바 모모이'가 밀레니엄 학원을 배경으로 게임을 만들었다고 자신만만하게 큰소리치며 선생이 게임을 시작하는 것으로 본 2차 창작 게임이 시작된다.

## 2. 시스템 환경 설정
- 폰트는 [`Pretendard`](https://github.com/orioncactus/pretendard)를 사용한다.
- 화면은 16:9 비율을 유지하며, 정해진 해상도로만 변경하게 한다.

## 3. 장면 설정
<b>주의: 해당 내용은 제작 도중 변경될 수 있다.</b>

### Setup 장면
- 게임에서 사용되는 에셋을 로드하고 애플리케이션 윈도우를 설정합니다.
- 에셋이 모두 로드되는 동안 검은색 화면을 출력합니다.
- 모든 에셋이 로드된 경우 다음 장면으로 변경 합니다.

### Entry 장면 
![IMG_INTRO_2.png](./files/IMG_INTRO_2.png) </br>
- 검은색 화면에서 페이드 인(Fade in)됩니다.
- 안내 문구는 화면 중앙에 위치합니다.
- 화면 오른쪽 하단에 유료 라이센스 라이브러리의 로고를 넣습니다.
- 일정 시간 이후 검은색 화면으로 페이드 아웃(Fade out)됩니다.

</br>

# Proposal (English Translation)
## 1. Story
Schale's teacher visits the Game Development Department at Millennium Science School. Because they asked to play the game they made and share the review. After talking with students from the Game Development Department Schale's teacher starts the game.

## 2. System Environment Settings
- Uses [`Pretendard`](https://github.com/orioncactus/pretendard) for the font.
- The screen maintains the 16:9 ratio and can only be changed to the specified resolution.

## 3. Scene Settings
<b>Note: The contents may be changed during production.</b>

### Setup Scene
- Loads assets used in the game and setup the application window.
- Display a black screen while all assets are loaded.
- When all assets are loaded, change to the next scene.

### Entry Scene
![IMG_INTRO_2.png](./files/IMG_INTRO_2.png)
- Fade in on a black screen.
- The guide text is located in the center of the screen.
- Put the logo of the paid license library on the bottom right of the screen.
- Fade out to a black screen after a certain period of time.
