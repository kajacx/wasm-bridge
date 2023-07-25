export const insecure = {
  getInsecureRandomBytes(len) {
    return random.getRandomBytes(len);
  },
  getInsecureRandomU64() {
    return random.getRandomU64();
  },
};

let insecureSeedValue1, insecureSeedValue2;

export const insecureSeed = {
  insecureSeed() {
    if (insecureSeedValue1 === undefined) {
      insecureSeedValue1 = random.getRandomU64();
      insecureSeedValue2 = random.getRandomU64();
    }
    return [insecureSeedValue1, insecureSeedValue2];
  },
};

export const random = {
  getRandomBytes(len) {
    // We don't really care about this impl, it will be overridden anyway
    const bytes = new Uint8Array(Number(len));
    return bytes;
  },

  getRandomU64() {
    // We don't really care about this impl, it will be overridden anyway
    return 0n;
  },

  insecureRandom() {
    if (insecureRandomValue1 === undefined) {
      insecureRandomValue1 = random.getRandomU64();
      insecureRandomValue2 = random.getRandomU64();
    }
    return [insecureRandomValue1, insecureRandomValue2];
  },
};
