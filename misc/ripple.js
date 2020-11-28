<!DOCTYPE html>
<html lang="">

<head>
    <meta charset="UTF-8">
    <title>Untitled Document</title>
    <meta name="Author" content="" />
    <style>
        .gridd {
            width: 50px;
            height: 50px;
/*                        border: solid 2px red;*/
            display: inline-block;
            position: relative;
            animation: colorer 9s ease-out;
            margin: 10px
        }

        .box {
            width: 10px;
            height: 10px;
            display: inline-block;
            margin: 2px;
                        border: solid 1px gray;
        }

        @keyframes colorer {
            0% {
                background-color: #0EFFFF;
            }

            50% {
                background-color: #0EFFA8;
            }

            80% {
                background-color: #0EFFA1;
            }

            100% {
                background-color: #0EFFA1;
            }
        }
    </style>
</head>

<body>
    <div class="parent">
    </div>

    <script src="js/dev/jquery.2.2.3.min.js"></script>
    <script>
        var gridT = 100;
        $(document).ready(function() {
            var t = 0
            var rot = Math.sqrt(gridT);
            var sub = 0;
            while (t < gridT) {
                if (t%rot == 0) {
                    sub++;
                   $(".parent").append(
                    `<div id=sub${sub} class="subparent${sub}"></div>`
                    ) 
                }
                $(`.subparent${sub}`).append(
                    `<div id=grid${t} class="box"></div>`
                )
                t++;
            }
            var t = 0;
            const inter = setInterval(function() {
                $(`#grid${t}`).addClass('gridd');
                t++;
                if (t === gridT) {
                    $(`.box`).removeClass('gridd');
                    clearInterval(inter);
                }
            }, 10);
            
        $('.box').click(function() {
            var gridder = [];
            console.log($(this)[0].id);
            var idd = $(this)[0].id.slice(4);
            console.log(idd);
            var parId = parseInt(idd);
            gridder.push(parId);
            appender(parId, gridder, rot)
            var i = 0;
            const colorer = setInterval(function() {
                
                appender(`${gridder[i]}`, gridder, rot);
                console.log(i, gridder.length, gridder);
                $(`#grid${gridder[i]}`).addClass('gridd');
                
                i++;
                if (i == gridder.length + 1) {
                    setTimeout(function() {
                        $(`.box`).removeClass('gridd');
                    }, 100)
                    clearInterval(colorer);
                }
            }, 10)
            console.log(gridder);
        })
        });
        function appender(parId, gridder, rot) {
            parId = parseInt(parId);
            if ((parId+1)%rot == 0) {
                let newBox = [parId - rot, parId - 1, parId + rot];
                gridAdder(gridder, newBox);
//                gridder.push(...[parId - rot, parId - 1, parId + rot]);
            }
            else if(parId%rot == 0) {
                let newBox = [parId + 1, parId - rot, parId + rot];
                gridAdder(gridder, newBox);
//                gridder.push(...[parId + 1, parId - rot, parId + rot]);
            }
            else {
                let newBox = [parId + 1, parId - rot, parId - 1, parId + rot];
                gridAdder(gridder, newBox);
//                gridder.push(...[parId + 1, parId - rot, parId - 1, parId + rot]);
            }
        } 
        function gridAdder(gridder, newBox){
                newBox.forEach(ele => {
                    if(!gridder.includes(ele)) {
                        if(ele >= 0 && ele < gridT) {
                            gridder.push(ele);
                        }
                    }
                })
        }
    </script>
</body>

</html>
