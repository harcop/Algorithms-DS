var spiralOrder = function(matrix) {
  const res = []
  while(matrix.length){
    const first = matrix.shift()
    res.push(...first)
    for(const m of matrix){
      let val = m.pop()
      if(val || val === 0)
        res.push(val)
        m.reverse()   
    }
    matrix.reverse()
  }
  return res
};

console.log(spiralOrder([[1,2,3],[4,5,0],[7,8,9]]))
