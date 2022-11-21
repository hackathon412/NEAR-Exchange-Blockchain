import request from '../utils/axios'

export function marketVolume(data) {
    return request({
        url: '/marketVolume',
        method: 'post',
        data
    })
}