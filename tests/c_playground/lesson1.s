;
; File generated by cc65 v 2.16 - Git 035baa4
;
	.fopt		compiler,"cc65 v 2.16 - Git 035baa4"
	.setcpu		"6502"
	.smart		on
	.autoimport	on
	.case		on
	.debuginfo	off
	.importzp	sp, sreg, regsave, regbank
	.importzp	tmp1, tmp2, tmp3, tmp4, ptr1, ptr2, ptr3, ptr4
	.macpack	longbranch
	.forceimport	__STARTUP__
	.export		_index
	.export		_TEXT
	.export		_PALETTE
	.export		_main

.segment	"RODATA"

_TEXT:
	.byte	$48,$65,$6C,$6C,$6F,$20,$57,$6F,$72,$6C,$64,$21,$00
_PALETTE:
	.byte	$1F
	.byte	$00
	.byte	$10
	.byte	$20

.segment	"BSS"

.segment	"BSS"
_index:
	.res	1,$00

; ---------------------------------------------------------------
; void __near__ main (void)
; ---------------------------------------------------------------

.segment	"CODE"

.proc	_main: near

.segment	"CODE"

;
; PPU_CTRL = 0;
;
	lda     #$00
	sta     $2000
;
; PPU_MASK = 0;
;
	sta     $2001
;
; PPU_ADDRESS = 0x3f;  // set an address in the PPU of 0x3f00
;
	lda     #$3F
	sta     $2006
;
; PPU_ADDRESS = 0x00;
;
	lda     #$00
	sta     $2006
;
; for(index = 0; index < sizeof(PALETTE); ++index){
;
	sta     _index
L004C:	lda     _index
	cmp     #$04
	bcs     L004D
;
; PPU_DATA = PALETTE[index];
;
	ldy     _index
	lda     _PALETTE,y
	sta     $2007
;
; for(index = 0; index < sizeof(PALETTE); ++index){
;
	inc     _index
	jmp     L004C
;
; PPU_ADDRESS = 0x21;   // set an address in the PPU of 0x21ca
;
L004D:	lda     #$21
	sta     $2006
;
; PPU_ADDRESS = 0xca;   // about the middle of the screen
;
	lda     #$CA
	sta     $2006
;
; for( index = 0; index < sizeof(TEXT); ++index ){
;
	lda     #$00
	sta     _index
L004E:	lda     _index
	cmp     #$0D
	bcs     L004F
;
; PPU_DATA = TEXT[index];
;
	ldy     _index
	lda     _TEXT,y
	sta     $2007
;
; for( index = 0; index < sizeof(TEXT); ++index ){
;
	inc     _index
	jmp     L004E
;
; PPU_ADDRESS = 0;
;
L004F:	lda     #$00
	sta     $2006
;
; PPU_ADDRESS = 0;
;
	sta     $2006
;
; SCROLL = 0;
;
	sta     $2005
;
; SCROLL = 0;
;
	sta     $2005
;
; PPU_CTRL = 0x90;  // screen is on, NMI on
;
	lda     #$90
	sta     $2000
;
; PPU_MASK = 0x1e;
;
	lda     #$1E
	sta     $2001
;
; while (1);
;
L004B:	jmp     L004B

.endproc

