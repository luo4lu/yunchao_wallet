import {Column, Entity, PrimaryColumn} from 'typeorm';
@Entity()
class Wallet{
    @PrimaryColumn()
    id:string;
    @Column()
    type:string;
    @Column()
    created:number;
    @Column()
    appid:string
    @Column("simple-json")
    extra: {}
    @Column()
    block:boolean
    @Column()
    encrypted_password:string;
    @Column("simple-array")
    bank_cards:{}[]
    @Column()
    reviewed:boolean
    @Column()
    available_balance:number
    @Column()
    withdrawable_balance:number
    @Column()
    freeze_balance:number
    @Column()
    channel:string
}

export default Wallet
