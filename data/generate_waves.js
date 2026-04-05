const fs = require('fs');

const counts      = [36,45,40,36,36,36,30,36,45,3, 54,45,45,26,36,45,35,45,36,3, 36,48,36,35,45,36,36,18,30,3, 15];
const bounties    = [3,3,4,5,5,5,6,6,5,51, 5,6,7,12,9,8,10,8,10,86, 10,9,11,11,9,12,12,23,14,123, 0];
const endRoundGold= [11,12,13,14,16,18,20,23,26,30, 35,40,45,50,55,60,70,80,90,100, 110,120,130,140,150,160,170,180,190,200, 0];
const recommendVal= [250,350,500,650,800, 1000,1200,1450,1600,1850, 2050,2400,2700,3100,3500, 4000,4500,5000,5500,6000, 6500,7100,7700,8500,9500, 10600,11800,13000,14000,15000, 0];

const armorTypes = {
  unarmored: [1,2,31],
  light: [5,7,10,13,16,19,21,25],
  medium: [3,8,12,14,18,24,27],
  heavy: [4,9,15,20,23,26,29],
  fortified: [6,11,17,22,28,30]
};

const attackTypes = {
  piercing: [1,4,7,12,19,21,25],
  normal: [2,3,9,14,15,23,26,27],
  magic: [5,8,13,16,18,24,29],
  siege: [6,11,17,22,28],
  chaos: [10,20,30,31]
};

const airWaves = [5,13,21,29];
const rangedWaves = [4,8,12,16,20,24,28,29];
const bossWaves = [10,20,30];

function getArmorType(wave) {
  for (const [type, waves] of Object.entries(armorTypes)) {
    if (waves.includes(wave)) return type;
  }
  return 'unarmored';
}

function getAttackType(wave) {
  for (const [type, waves] of Object.entries(attackTypes)) {
    if (waves.includes(wave)) return type;
  }
  return 'normal';
}

const waves = [];
for(let w = 1; w <= 30; w++) {
  let incomeCap = Math.floor(0.025 * Math.pow(w, 3) + 0.05 * Math.pow(w, 2) + 4 * w + 20);
  let buildTimer = Math.floor(40 + (w / 2));
  
  waves.push({
    wave: w,
    count: counts[w-1],
    bounty: bounties[w-1],
    armor_type: getArmorType(w),
    attack_type: getAttackType(w),
    is_air: airWaves.includes(w),
    is_ranged: rangedWaves.includes(w),
    is_boss: bossWaves.includes(w),
    end_round_gold: endRoundGold[w-1],
    recommend_value: recommendVal[w-1],
    build_timer_seconds: buildTimer,
    income_cap: incomeCap
  });
}

fs.writeFileSync('waves.json', JSON.stringify(waves, null, 2));
console.log('Successfully generated waves.json');
