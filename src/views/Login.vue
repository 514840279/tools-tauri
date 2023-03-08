<template>
  <div class="bg-box">
    <el-card class="box-card">
      <el-form ref="loginFormRef" :model="loginForm" status-icon :rules="rules" size="large" label-width="80px" class="loginForm">
        <h1 class="title">{{ title }}</h1>
        <el-form-item label="用户名" prop="name">
          <el-input v-model="loginForm.name" autocomplete="off" />
        </el-form-item>
        <el-form-item label="密码" prop="password">
          <el-input v-model="loginForm.password" type="password" autocomplete="off" />
        </el-form-item>
        <el-form-item>
          <el-button class="sub-btn" type="primary" @click="submitForm()">确定</el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { LoginForm } from '../interface/Login'

const title: String = "登录";

let loginForm = ref<LoginForm>({
  name: "wth",
  password: "123456",
});

const rules = {
  name: [
    { required: true, message: "用户名不能为空", trigger: "blur" },
    { min: 4, max: 50, message: "长度在 4 to 50", trigger: "change" },
  ],
  password: [
    { required: true, message: "密码不能为空", trigger: "blur" },
    { min: 6, max: 50, message: "长度在 6 to 50", trigger: "change" },
  ],
};

function submitForm() {
  console.log(loginForm.value);


}
</script>

<style lang="scss" scoped>
body {
  margin: 0px;
  padding: 0px;

}

.bg-box {
  width: 100%;
  height: 100%;
  // background-image: url("../assets/log-background/bg1.jpg");
  background-size: cover;
  background-attachment: fixed;
  /*关键*/
  background-position: top;
  overflow-y: hidden;
  min-height: 697px;
  padding-top: 180px;
  zoom: 1;

  .box-card {
    width: 45%;
    margin: auto;
  }

  .loginForm {
    width: 450px;
    height: 300px;
    background-color: white;
    padding-top: 20px;
    padding-right: 25px;
    margin: auto;
    border-radius: 15px;

    .title {
      text-align: center;
      margin: 20px 0;
      color: rgb(11, 26, 39);
    }

    .sub-btn {
      width: 100%;
    }
  }
}
</style>
