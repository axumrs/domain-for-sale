<script setup lang="ts">
import { Icon } from "@iconify/vue";
import VueHcaptcha from "@hcaptcha/vue3-hcaptcha";
import { reactive, ref } from "vue";
import Toast from "./components/Toast.vue";

const frm = reactive({ currency: "USDT", name: "", email: "", captcha: "" });
const expandCurrency = ref(false);
const msg = ref("");
const siteKey = import.meta.env.VITE_HCAPTCHA_SITE_KEY;

const handleSubmit = async () => {
  const name = frm.name.trim();
  const email = frm.email.trim();
  const captcha = frm.captcha.trim();

  if (!(name.length >= 3 && name.length <= 50)) {
    msg.value = "请输入正确的联系人/Please enter your full name";
    return;
  }
  if (!(email.length >= 3 && email.length <= 255 && email.indexOf("@") > 0)) {
    msg.value = "请输入正确的邮箱/Please enter your email";
    return;
  }
  if (!["USDT", "CNY"].includes(frm.currency)) {
    msg.value = "请选择货币/Please select currency";
    return;
  }
  if (captcha.length < 16) {
    msg.value = "请完成人机验证/Please complete Captcha";
    return;
  }
};
</script>

<template>
  <section class="grow bg-[url('bg-dot.png')]">
    <div class="bg-dark/50 p-3 lg:min-h-full relative">
      <div
        class="lg:absolute lg:top-1/2 lg:left-1/2 lg:-translate-x-1/2 lg:-translate-y-1/2 flex flex-col lg:flex-row lg:justify-between lg:items-start lg:gap-x-4"
      >
        <div class="lg:grow lg:shrink-0">
          <div
            class="text-[2.8rem] flex flex-col lg:flex-row justify-start items-center"
          >
            <p class="lg:after:content-['。'] lg:after:text-price">域名转让</p>
            <p
              class="lg:after:content-['.'] lg:after:text-price after:text-[3rem]"
            >
              Domain for sale
            </p>
          </div>
          <div class="text-2xl my-6">
            <p>
              <span class="text-price">axum.rs</span>
              现已开始转让，赶快和我们联系吧。
            </p>
            <p>
              <span class="text-price">axum.rs</span> is now available for sale,
              contact us now.
            </p>
          </div>
          <hr />
          <div>
            <div
              class="text-price text-[4rem] font-semibold flex justify-start items-center gap-x-1"
            >
              <div class="text-[3rem]">
                <Icon icon="mingcute:tether-usdt-line" />
              </div>
              <div><span>702</span> <span class="text-lg">USDT</span></div>
            </div>
            <div
              class="text-price text-[4rem] font-semibold flex justify-start items-center gap-x-1"
            >
              <div class="text-[3rem]">
                <Icon icon="mingcute:currency-cny-line" />
              </div>
              <div>
                <span>5000</span> <span class="text-lg">元人民币 / CNY</span>
              </div>
            </div>
          </div>
        </div>
        <form
          autocomplete="off"
          class="bg-dark lg:min-h-full p-6 lg:grow lg:shrink-0 flex flex-col gap-y-3"
          @submit.prevent="handleSubmit"
        >
          <div
            class="text-xl flex flex-col lg:flex-row lg:justify-start lg:items-center gap-x-1"
          >
            <div>提交你的意向</div>
            <div>SUBMIT YOUR OFFER</div>
          </div>
          <label class="flex flex-col w-full gap-y-1">
            <div>联系人 / Full Name</div>
            <div class="bg-primary px-4 py-2 text-dark">
              <input
                type="text"
                class="w-full bg-transparent outline-none block"
                placeholder="联系人 / Full Name"
                v-model="frm.name"
              />
            </div>
          </label>
          <label class="flex flex-col w-full gap-y-1">
            <div>邮箱 / Email</div>
            <div class="bg-primary px-4 py-2 text-dark">
              <input
                type="email"
                class="w-full bg-transparent outline-none block"
                placeholder="邮箱 / Email"
                v-model="frm.email"
              />
            </div>
          </label>
          <label class="flex flex-col w-full gap-y-1">
            <div>货币 / Currency</div>
            <div class="bg-primary px-4 py-2 text-dark relative">
              <div
                class="flex justify-between items-center cursor-pointer"
                @click="expandCurrency = !expandCurrency"
              >
                <div>
                  <div
                    class="flex justify-start items-center gap-x-1"
                    v-if="frm.currency === 'USDT'"
                  >
                    <div><Icon icon="mingcute:tether-usdt-line" /></div>
                    <div>702 USDT</div>
                  </div>
                  <div class="flex justify-start items-center gap-x-1" v-else>
                    <div><Icon icon="mingcute:currency-cny-line" /></div>
                    <div>5000 元人民币/CNY</div>
                  </div>
                </div>
                <div>
                  <Icon
                    icon="gravity-ui:caret-down"
                    class="transition-all"
                    :class="{ 'rotate-180': expandCurrency }"
                  />
                </div>
              </div>
              <ul
                class="absolute bg-primary left-0 w-full top-11 flex flex-col gap-y-2 transition-all"
                :class="{
                  'opacity-100 visible': expandCurrency,
                  'opacity-0 invisible': !expandCurrency,
                }"
              >
                <li
                  class="flex justify-start items-center gap-x-1 px-4 py-2 cursor-pointer"
                  @click="
                    () => {
                      frm.currency = 'USDT';
                      expandCurrency = false;
                    }
                  "
                  :class="{ 'bg-price': frm.currency === 'USDT' }"
                >
                  <div><Icon icon="mingcute:tether-usdt-line" /></div>
                  <div>702 USDT</div>
                </li>
                <li
                  class="flex justify-start items-center gap-x-1 px-4 py-2 cursor-pointer"
                  @click="
                    () => {
                      frm.currency = 'CNY';
                      expandCurrency = false;
                    }
                  "
                  :class="{ 'bg-price': frm.currency === 'CNY' }"
                >
                  <div><Icon icon="mingcute:currency-cny-line" /></div>
                  <div>5000 元人民币/CNY</div>
                </li>
              </ul>
            </div>
          </label>

          <div>
            <vue-hcaptcha :sitekey="siteKey"></vue-hcaptcha>
          </div>

          <div>
            <button
              type="submit"
              class="flex justify-center items-center gap-x-2 px-4 py-3 w-full bg-price text-dark disabled:bg-price/70 disabled:text-gray-600 disabled:cursor-not-allowed"
            >
              <div>
                <Icon icon="mingcute:loading-line" class="animate-spin" />
              </div>
              <div>提交</div>
              <div>SUBMIT</div>
            </button>
          </div>
        </form>
      </div>
    </div>
  </section>
  <footer
    class="w-full bg-dark/90 shrink-0 px-6 py-6 flex flex-col gap-y-4 lg:flex-row lg:gap-y-0 lg:justify-between lg:items-center"
  >
    <ul class="flex justify-start items-center gap-x-4">
      <li>
        USDT
        <span class="text-xs text-gray-400">TRC20</span>:
        <span
          class="font-mono p-1 border border-dotted rounded border-gray-600 bg-price/50"
          >TPGEtKJmPJU3naosCcRrVReE2ckFhE9sYM</span
        >
      </li>
    </ul>

    <div class="text-xs uppercase">
      &copy; 2024 <span class="text-price">axum.rs</span>, all rights reserved.
    </div>
  </footer>

  <Toast v-if="msg" @closed="msg = ''">
    <div v-for="m in msg.split('/')" :key="m">{{ m }}</div>
  </Toast>
</template>
