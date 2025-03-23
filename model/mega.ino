// 使用 Arduino Mega 2560
#include <Arduino.h>
#include <Servo.h>

Servo tary_up, tary_down, zip_servo;                                  // 底部元件
Servo dam_board_One, dam_board_Two, dam_board_Three, dam_board_Four;  // 侧板
Servo pull1, pull2, claw;                                             // 爪子

/*
pull_1 冲着舵机正面看，左手边的
pull_2 冲着舵机正面看，右手边的
claw 爪子控制
*/

// 定义舵机引脚
#define tary_up_pin 8
#define tary_down_pin 9


#define dam_board_One_pin 10
#define dam_board_Two_pin 11
#define dam_board_Three_pin 12
#define dam_board_Four_pin 13

#define pull1_pin 6
#define pull2_pin 7
#define claw_pin 5

// 120 是抓合的时候
#define claw_init_degree 110
#define claw_end_degree 86
#define claw_open_degree 150

// 伸缩爪子
#define pull1_init_degree 130
#define pull1_end_degree 180
#define pull2_init_degree 50
#define pull2_end_degree 0

// 云台
#define tary_up_init_degree 82
#define tary_down_init_degree 86

// 侧板角度
#define dam_board_open_degree 90
#define dam_board_close_degree 42  // TDDO:加以修改

#define buttonPin 2  // 假设按钮连接在数字引脚2


// 函数定义
void init_servo();
void process1(int types);
void process2(int types);

int num1, num2 = 0;
void setup() {
  // 初始化串口通信
  Serial.begin(9600);
  // 初始化舵机
  tary_up.attach(tary_up_pin);
  tary_down.attach(tary_down_pin);
  dam_board_One.attach(dam_board_One_pin);
  dam_board_Two.attach(dam_board_Two_pin);
  dam_board_Three.attach(dam_board_Three_pin);
  dam_board_Four.attach(dam_board_Four_pin);
  pull1.attach(pull1_pin);
  pull2.attach(pull2_pin);
  claw.attach(claw_pin);

  //初始化推杆
  pinMode(A0, OUTPUT);
  pinMode(A1, OUTPUT);
  digitalWrite(A0, HIGH);
  digitalWrite(A1, LOW);
  pinMode(buttonPin, INPUT_PULLUP);
  dam_board_One.write(dam_board_open_degree - 20);


  init_servo();  // 重置舵机
}
void init_servo() {
  Serial.println("state0 begin");  // 初始化
  tary_up.write(tary_up_init_degree);
  tary_down.write(tary_down_init_degree);
  delay(300);
  dam_board_One.write(dam_board_close_degree - 5);  //  90-180
  dam_board_Three.write(dam_board_close_degree - 3);
  dam_board_Two.write(dam_board_close_degree - 4);
  dam_board_Four.write(dam_board_close_degree - 2);
  delay(100);
  claw.write(claw_init_degree);  // 测试，爪子舵机
  delay(50);
  pull1.write(pull1_init_degree);
  pull2.write(pull2_init_degree);
  delay(50);
}

void loop() {
  if (Serial.available() > 0) {
    num1 = Serial.parseInt();  // 任务1 任务2
    num2 = Serial.parseInt();  // 垃圾种类

    // 打印接收到的数字
    Serial.print("Received number 1: ");
    Serial.println(num1);
    Serial.print("Received number 2: ");
    Serial.println(num2);
    switch (num1) {
      case 1:
        process1(num2);
        break;
      case 2:
        process2(num2);
        break;
      default:
        break;
    }
  }
   int buttonState = digitalRead(buttonPin);
    if (buttonState == LOW) {  // 如果引脚为高电平（按钮按下）
      dam_board_One.write(dam_board_open_degree - 15);
      dam_board_Two.write(dam_board_open_degree - 15);
      dam_board_Three.write(dam_board_open_degree - 15);
      dam_board_Four.write(dam_board_open_degree - 15);
    }
}
void process1(int types) {
  // // 打开侧板
  // dam_board_One.write(dam_board_close_degree + 5);
  // dam_board_Two.write(dam_board_close_degree + 5);
  // dam_board_Three.write(dam_board_close_degree + 5);
  // dam_board_Four.write(dam_board_close_degree + 5);
  // delay(100);
  // dam_board_One.write(dam_board_close_degree - 8);
  // dam_board_Two.write(dam_board_close_degree - 8);
  // dam_board_Three.write(dam_board_close_degree - 8);
  // dam_board_Four.write(dam_board_close_degree - 8);
  // delay(500);
  dam_board_One.write(dam_board_open_degree - 15);
  dam_board_Two.write(dam_board_open_degree - 15);
  dam_board_Three.write(dam_board_open_degree - 18);
  dam_board_Four.write(dam_board_open_degree - 15);
  delay(1000);

  switch (types) {
    case 1:  // 根据实际情况修改
      tary_down.write(tary_down_init_degree + 45);
      delay(500);
      tary_up.write(tary_up_init_degree - 45);
      break;
    case 2:
      tary_down.write(tary_down_init_degree - 45);
      delay(500);
      tary_up.write(tary_up_init_degree + 45);
      break;
    case 3:
      tary_down.write(tary_down_init_degree - 45);
      delay(500);
      tary_up.write(tary_up_init_degree - 45);
      break;
    case 4:
      tary_down.write(tary_down_init_degree + 45);
      delay(500);
      tary_up.write(tary_up_init_degree + 45);
      break;
    default:
      init_servo();
      break;
  }
  while (true) {  // 无限循环
    if (Serial.available() > 0) {
      char receivedChar = Serial.read();
      if (receivedChar == 'q') {
        break;
      }
    }
  }
  init_servo();
}

void process2(int types) {
  // 等待 2.5s 步进移动到垃圾上方
  delay(500);
  // 连杆下降
  pull1.write(pull1_end_degree);
  pull2.write(pull2_end_degree);
  claw.write(claw_open_degree);
  delay(1500);
  // 爪子合拢
  claw.write(claw_end_degree);
  delay(1000);
  // 连杆上升
  pull1.write(pull1_init_degree);
  pull2.write(pull2_init_degree);
  delay(1000);

  dam_board_One.write(dam_board_open_degree);
  dam_board_Two.write(dam_board_open_degree);
  delay(100);
  dam_board_Three.write(dam_board_open_degree);
  dam_board_Four.write(dam_board_open_degree);
  delay(100);

  // // **无限等待串口输入**
  // Serial.println("Waiting for input...");
  while (true) {  // 无限循环
    if (Serial.available() > 0) {
      char receivedChar = Serial.read();  // 读取一个字符

      if (receivedChar == 'q') {
        Serial.println("Received 'q', staying in current state.");
        // 放下推杆
        tary_down.write(tary_down_init_degree + 45);
        delay(200);
        pull1.write(pull1_end_degree);
        pull2.write(pull2_end_degree);
        delay(500);
        // 爪子张开
        claw.write(claw_open_degree);
        delay(2000);
        claw.write(claw_init_degree);
        // 推杆上升
        pull1.write(pull1_init_degree);
        pull2.write(pull2_init_degree);
        delay(500);
        init_servo();
        break;  // 收到 'q'，跳出循环，回到 loop() 的开头等待新数据
      }
    }
  }
}
