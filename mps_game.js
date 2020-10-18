const ele = document.createElement("div");
        ele.setAttribute("class", "item boxer");
        
        console.log(ele);
//        const _t = document.getElementsByClassName("tree");
        const _t = document.getElementsByClassName("root2");
        if (_t[0] === undefined) {
            console.log(console.log(_t[0]), 'here');
        } 
        const hack = 2;
        console.log(`${hack+2}`)
        
        const tree = {
            value: 1,
            left: {
                value: 2,
                left:{
                    value: 4,
                    left: null,
                    right: null
                },
                right: {
                    value: 5,
                    left: null,
                    right: null
                }
            },
            right: {
                value: 3,
                left: {
                    value: 6,
                    right: null,
                    left: null
                },
                right: {
                    value: 7,
                    left: null,
                    right: null
                }
            }
        };
        
        function getH (tree) {
            let current = tree;
            let h = 1;
            while (current.left !== null) {
                current = current.left;
                h++;
            }
            return h;
        }
        
        console.log(getH(tree), "treeHeight");
        let treeH = getH(tree);
        
        let marginLeft = 30;
        function rs(node, stage, side) {
            const { value, left, right } = node;
            const parentClass = `c${stage}`
            let newParent = document.getElementsByClassName(parentClass);
            if (newParent[0] === undefined) {
                newParent = document.createElement("div");
                newParent.setAttribute('class', parentClass);
                let myMarginLeft = marginLeft * ((treeH+1)/stage);
                newParent.setAttribute("style", `margin-left: ${myMarginLeft}px`)
                
                $('.tree').append(newParent);
            }
            const newChild = document.createElement("div");
            newChild.setAttribute('class', "box");
            const txtNode = document.createTextNode(`${value}`);
            newChild.append(txtNode);
            if (side === "right") {
                newChild.setAttribute("style", `margin-left: ${marginLeft}px`)
            }
            $(`.${parentClass}`).append(newChild);
            
            if (left !== null) {
                rs(left, stage*2, "left");
            }
            if (right !== null) {
                rs(right, stage*2, "right");
            }
        }
        
        rs(tree, 1, "center");
        const _n = document.getElementsByClassName("tree");
        console.log(_n[0]);