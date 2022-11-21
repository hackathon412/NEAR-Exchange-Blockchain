import tokenList from "../config/tokenList.js"
import axios from 'axios'

function getTokenAddress(symbol) {
    const tokenCfg = tokenList.tokens.filter(token => token.symbol == symbol);
    return tokenCfg[0].address
}

function getTokenCfg(symbol) {
    const tokenCfg = tokenList.tokens.filter(token => token.symbol == symbol);
    return tokenCfg[0]
}

function formatNumber(num, decimal = 0) {
    num = String(num)
    if (num === '' || num === '0') return num
    const arr = num.split('.')
    const integerPart = arr[0]
    const decimalPart = arr[1] || ''
    const len = integerPart.length

    let str = ''

    if (len > 2) {
        integerPart.split('').forEach((item, index) => {
            if (index > 0 && (len - index) % 3 === 0) str += ''
            str += item
        })
    } else {
        str = integerPart
    }

    if (decimal === 0) return str
    str += '.'

    const decimalLen = decimalPart.length
    if (decimal === decimalLen) {
        str += decimalPart
    } else if (decimal > decimalLen) {
        str += decimalPart + new Array(decimal - decimalLen).fill('0').join('')
    } else {
        str += decimalPart.substr(0, decimal)
    }

    return str
}

async function getMarketPrice(token) {
    const response = await axios.get(`https://api.coingecko.com/api/v3/coins/markets?vs_currency=usd&ids=${getTokenCfg(token).id}`)
    let { data } = response
    return formatNumber(data[0].current_price, 4)
}

export {
    getTokenAddress,
    formatNumber,
    getTokenCfg,
    getMarketPrice
}