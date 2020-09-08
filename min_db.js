/* eslint-disable prefer-destructuring */
const namer = [
    {
        country: "Columbia",
        capital: "Bogòta"
    },
    {
        country: "Nigeria",
        capital: "Abuja"
    }
];

function DB (namers) {
    this.db = namers;
    this.find = () => {
        return namers;
    };
    this.findOne = (valueObj) => {
        const key = Object.getOwnPropertyNames(valueObj)[0];
        return this.db.find(ele => ele[key].toLowerCase() === valueObj[key].toLowerCase());
    };
    this.updateOne = (valueObj, valueUpdate) => {
        const key = Object.getOwnPropertyNames(valueObj);
        const update = Object.getOwnPropertyNames(valueUpdate);
        let inn = 0;
        this.db.forEach((element, index) => {
            if (element[key] === valueObj[key[0]]) {
                inn = index;
                const newKey = Object.getOwnPropertyNames(element);
                newKey.forEach(ele => {
                    if (update.includes(ele)) {
                        this.db[index][ele] = valueUpdate[ele];
                    }
                });
            }
        });
        return this.db[inn];
    };
}

const db = new DB(namer);
console.log(db.find());
console.log(db.findOne({ country: "nigeria" }));
console.log(db.updateOne({ country: "Nigeria" }, { capital: 'Lagos' }));