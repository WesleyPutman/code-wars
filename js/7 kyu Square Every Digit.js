function squareDigits(num){
  let squaredNumber = Number(num.toString().split('').map(c => Number(c)).map(x => x ** 2).join(''));
  return squaredNumber;
}
console.log(squareDigits(3212))