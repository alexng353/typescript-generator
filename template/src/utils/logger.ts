import pino from "pino";
import pretty from "pino-pretty";

function GetEmojiByTime(stamp: number) {
  const time = new Date(stamp)
    .toLocaleTimeString("en-CA", {
      timeZone: "America/Vancouver",
    })
    .split(":");

  const hour = parseInt(time[0]);
  const minute = parseInt(time[1]);

  const stringmoji = "🕛🕧🕐🕜🕑🕝🕒🕞🕓🕟🕔🕠🕕🕡🕖🕢🕗🕣🕘🕤🕙🕥🕚🕦";
  const moji = stringmoji.match(/.{1,2}/g) || [];
  const moji00 = moji.filter((_, index) => index % 2 === 0);
  const moji30 = moji.filter((_, index) => index % 2 !== 0);

  if (minute < 30) {
    return moji00[hour == 12 ? 0 : hour];
  } else {
    return moji30[hour == 12 ? 0 : hour];
  }
}

const stream = pretty({
  customPrettifiers: {
    time: (timestamp: any) =>
      `${GetEmojiByTime(timestamp)} ${new Date(timestamp).toLocaleString(
        "en-CA",
        {
          timeZone: "America/Vancouver",
        }
      )}`,
  },
  colorize: true,
});

const logger = pino({}, stream);

export default logger;