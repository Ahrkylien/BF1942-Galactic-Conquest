subshader "Small_Bomb_M1_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 12.5;
	transparent true;
	twosided true;
	blendSrc sourceAlpha;
	blendDest one;
	texture "texture/Weapons/tiebomb";
}
