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
    const bytes = new Uint8Array(Number(len));
    // TODO: fill with random data
    return bytes;
  },

  getRandomU64() {
    // TODO: return random value instead
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
