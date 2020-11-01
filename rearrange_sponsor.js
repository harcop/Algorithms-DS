const sponsorCards = [2, 1, 3, 4, 1];
function rearrangeSponsor() {
  const newCards = [];
  const data = new Array(...sponsorCards);
  let i = 0;
  while (data.length) {
    if (i % 5 === 0) {
      newCards.push({
        sponsorName: "advert"
      });
      i += 1;
      continue;
    }
    newCards.push(data[0]);
    data.splice(0, 1);
    i += 1;
  }
  console.log(newCards, "card here", data, "bomber", sponsorCards);
  return newCards;
}

console.log(rearrangeSponsor());
